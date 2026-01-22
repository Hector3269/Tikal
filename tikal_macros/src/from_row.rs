use heck::ToSnakeCase;
use proc_macro_error::abort;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};
use crate::utils::extract_column_name;
use crate::type_conversion::generate_from_value;

pub fn expand_from_row_derive(input: DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(f), .. }) => &f.named,
        _ => abort!(input, "FromRow only supports structs with named fields"),
    };

    let assignments = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let column_name =
            extract_column_name(field).unwrap_or_else(|| field_name.to_string().to_snake_case());

        let expr = generate_from_value(field_name, &field.ty, &column_name, struct_name);

        quote! { #field_name: #expr }
    });

    quote! {
        impl tikal::domain::model::FromRow for #struct_name {
            fn from_row(
                row: std::collections::HashMap<String, tikal::domain::value_objects::Value>
            ) -> tikal::domain::TikalResult<Self> {
                Ok(Self {
                    #(#assignments,)*
                })
            }
        }
    }
}