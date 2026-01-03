use syn::{Attribute, Field};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ColumnAttr {
    pub name: Option<String>,
    pub is_primary_key: bool,
    pub is_auto_increment: bool,
    pub is_unique: bool,
    pub check_constraint: Option<String>,
}

impl Default for ColumnAttr {
    fn default() -> Self {
        Self {
            name: None,
            is_primary_key: false,
            is_auto_increment: false,
            is_unique: false,
            check_constraint: None,
        }
    }
}

#[allow(unused)]
impl ColumnAttr {
    pub fn parse_from_field(field: &Field) -> syn::Result<Self> {
        let mut attr = Self::default();

        for a in &field.attrs {
            if a.path().is_ident("column") {
                attr.parse_nested(a)?;
            }
        }

        Ok(attr)
    }

    fn parse_nested(&mut self, attr: &Attribute) -> syn::Result<()> {
        attr.parse_nested_meta(|meta| {
            if let Some(ident) = meta.path.get_ident() {
                match ident.to_string().as_str() {
                    "primary_key" => self.is_primary_key = true,
                    "auto_increment" => self.is_auto_increment = true,
                    "unique" => self.is_unique = true,
                    "name" => {
                        let lit: syn::LitStr = meta.value()?.parse()?;
                        self.name = Some(lit.value());
                    }
                    "check" => {
                        let lit: syn::LitStr = meta.value()?.parse()?;
                        self.check_constraint = Some(lit.value());
                    }
                    _ => {}
                }
            }
            Ok(())
        })
    }
}
