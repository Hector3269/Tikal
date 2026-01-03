use syn::{Attribute, ItemStruct};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TableAttr {
    pub name: Option<String>,
    pub scopes: Vec<ScopeAttr>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ScopeAttr {
    pub name: syn::Ident,
    pub condition: String,
}

#[allow(unused)]
impl TableAttr {
    pub fn parse_from_struct(item: &ItemStruct) -> syn::Result<Self> {
        let mut attr = Self {
            name: None,
            scopes: Vec::new(),
        };

        for a in &item.attrs {
            if a.path().is_ident("table") {
                attr.parse_table_attr(a)?;
            } else if a.path().is_ident("scope") {
                attr.parse_scope_attr(a)?;
            }
        }

        Ok(attr)
    }

    fn parse_table_attr(&mut self, attr: &Attribute) -> syn::Result<()> {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("name") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                self.name = Some(lit.value());
            }
            Ok(())
        })
    }

    fn parse_scope_attr(&mut self, attr: &Attribute) -> syn::Result<()> {
        attr.parse_nested_meta(|meta| {
            let name = meta
                .path
                .get_ident()
                .ok_or(meta.error("expected scope name"))?
                .clone();
            let condition: syn::LitStr = meta.value()?.parse()?;

            self.scopes.push(ScopeAttr {
                name,
                condition: condition.value(),
            });
            Ok(())
        })
    }
}
