use syn::{Attribute, Field, Type};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum RelationType {
    HasOne,
    HasMany,
    BelongsTo,
    BelongsToMany,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RelationAttr {
    pub rel_type: RelationType,
    pub related_model: Type,
    pub foreign_key: Option<String>,
    pub local_key: Option<String>,
    pub pivot_table: Option<String>,
    pub foreign_pivot_key: Option<String>,
    pub related_pivot_key: Option<String>,
    pub on_delete: Option<String>,
    pub on_update: Option<String>,
}

#[allow(unused)]
impl RelationAttr {
    pub fn parse_from_field(field: &Field) -> syn::Result<Vec<Self>> {
        let mut relations = Vec::new();

        for attr in &field.attrs {
            if let Some(rel) = Self::try_parse_single(attr, field)? {
                relations.push(rel);
            }
        }

        Ok(relations)
    }

    fn try_parse_single(attr: &Attribute, field: &Field) -> syn::Result<Option<Self>> {
        let rel_type = match attr.path().get_ident().map(|i| i.to_string()) {
            Some(ref s) if s == "has_many" => Some(RelationType::HasMany),
            Some(ref s) if s == "has_one" => Some(RelationType::HasOne),
            Some(ref s) if s == "belongs_to" => Some(RelationType::BelongsTo),
            Some(ref s) if s == "belongs_to_many" => Some(RelationType::BelongsToMany),
            Some(ref s) if s == "relation" => return Self::parse_relation_attr(attr, field),
            _ => None,
        };

        if let Some(rt) = rel_type {
            let mut relation = Self {
                rel_type: rt,
                related_model: Self::get_inner_type(&field.ty),
                foreign_key: None,
                local_key: None,
                pivot_table: None,
                foreign_pivot_key: None,
                related_pivot_key: None,
                on_delete: None,
                on_update: None,
            };

            attr.parse_nested_meta(|meta| {
                relation.parse_meta(&meta)?;
                Ok(())
            })?;

            Ok(Some(relation))
        } else {
            Ok(None)
        }
    }

    fn parse_relation_attr(attr: &Attribute, field: &Field) -> syn::Result<Option<Self>> {
        let mut rel_type = None;
        let mut related_model = None;
        let mut relation = Self {
            rel_type: RelationType::BelongsTo,
            related_model: Self::get_inner_type(&field.ty),
            foreign_key: None,
            local_key: None,
            pivot_table: None,
            foreign_pivot_key: None,
            related_pivot_key: None,
            on_delete: None,
            on_update: None,
        };

        attr.parse_nested_meta(|meta| {
            if let Some(ident) = meta.path.get_ident() {
                match ident.to_string().as_str() {
                    "belongs_to" => {
                        rel_type = Some(RelationType::BelongsTo);
                        let lit: syn::LitStr = meta.value()?.parse()?;
                        related_model = Some(syn::parse_str(&lit.value())?);
                    }
                    "has_many" => {
                        rel_type = Some(RelationType::HasMany);
                        let lit: syn::LitStr = meta.value()?.parse()?;
                        related_model = Some(syn::parse_str(&lit.value())?);
                    }
                    "has_one" => {
                        rel_type = Some(RelationType::HasOne);
                        let lit: syn::LitStr = meta.value()?.parse()?;
                        related_model = Some(syn::parse_str(&lit.value())?);
                    }
                    "belongs_to_many" => {
                        rel_type = Some(RelationType::BelongsToMany);
                        let lit: syn::LitStr = meta.value()?.parse()?;
                        related_model = Some(syn::parse_str(&lit.value())?);
                    }
                    _ => relation.parse_meta(&meta)?,
                }
            }
            Ok(())
        })?;

        if let (Some(rt), Some(rm)) = (rel_type, related_model) {
            relation.rel_type = rt;
            relation.related_model = rm;
            Ok(Some(relation))
        } else {
            Ok(None)
        }
    }

    fn parse_meta(&mut self, meta: &syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if let Some(ident) = meta.path.get_ident() {
            match ident.to_string().as_str() {
                "foreign_key" | "from" => {
                    self.foreign_key = Some(meta.value()?.parse::<syn::LitStr>()?.value())
                }
                "local_key" | "to" => {
                    self.local_key = Some(meta.value()?.parse::<syn::LitStr>()?.value())
                }
                "pivot_table" => {
                    self.pivot_table = Some(meta.value()?.parse::<syn::LitStr>()?.value())
                }
                "foreign_pivot_key" => {
                    self.foreign_pivot_key = Some(meta.value()?.parse::<syn::LitStr>()?.value())
                }
                "related_pivot_key" => {
                    self.related_pivot_key = Some(meta.value()?.parse::<syn::LitStr>()?.value())
                }
                "on_delete" => self.on_delete = Some(meta.value()?.parse::<syn::LitStr>()?.value()),
                "on_update" => self.on_update = Some(meta.value()?.parse::<syn::LitStr>()?.value()),
                _ => {}
            }
        }
        Ok(())
    }

    fn get_inner_type(ty: &Type) -> Type {
        match ty {
            Type::Path(tp) => {
                let seg = match tp.path.segments.last() {
                    Some(seg) => seg,
                    None => return ty.clone(),
                };

                match &seg.arguments {
                    syn::PathArguments::AngleBracketed(args) => {
                        if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                            if seg.ident == "Option" || seg.ident == "Vec" {
                                return Self::get_inner_type(inner);
                            }
                        }
                    }
                    _ => {}
                }
                ty.clone()
            }
            _ => ty.clone(),
        }
    }
}
