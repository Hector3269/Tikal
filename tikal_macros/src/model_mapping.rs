use heck::ToSnakeCase;
use proc_macro_error::abort;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};
use crate::utils::extract_column_name;

pub fn expand_model_mapping_derive(input: DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => abort!(input, "ModelMapping only supports structs with named fields"),
    };

    let mut inserts = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap().to_string();
        let column_name = extract_column_name(field)
            .unwrap_or_else(|| field_name.to_snake_case());

        inserts.push(quote! {
            mappings.insert(#field_name.to_string(), #column_name.to_string());
        });
    }

    quote! {
        impl tikal::domain::model::ModelMapping for #struct_name {
            fn column_mappings() -> std::collections::HashMap<String, String> {
                let mut mappings = std::collections::HashMap::new();
                #(#inserts)*
                mappings
            }

            fn field_to_column(field: &str) -> Option<String> {
                Self::column_mappings().get(field).cloned()
            }

            fn column_to_field(column: &str) -> Option<String> {
                Self::column_mappings()
                    .iter()
                    .find(|(_, v)| *v == column)
                    .map(|(k, _)| k.clone())
            }
        }
    }
}