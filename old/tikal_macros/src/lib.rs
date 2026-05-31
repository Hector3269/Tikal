use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

mod entity;
mod from_row;
mod model_mapping;
mod utils;
mod type_conversion;

#[proc_macro_derive(Entity, attributes(table_name, primary_key, column_name))]
#[proc_macro_error]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    entity::expand_entity_derive(input).into()
}

#[proc_macro_derive(FromRow, attributes(column_name))]
#[proc_macro_error]
pub fn derive_from_row(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    from_row::expand_from_row_derive(input).into()
}

#[proc_macro_derive(ModelMapping, attributes(column_name))]
#[proc_macro_error]
pub fn derive_model_mapping(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    model_mapping::expand_model_mapping_derive(input).into()
}