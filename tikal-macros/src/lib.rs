mod model;

use proc_macro::TokenStream;

// the example use api
//#[derive(Model, ModelMapping, Entity)]
//#[active_model("users")]
//pub struct User {
//    pub id: i64,
//   pub name: String,
//}

#[proc_macro_derive(ModelMapping, attributes(table, column, relation))]
pub fn model_mapping_derive(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(Model)]
pub fn model_derive(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn active_model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_derive(Entity, attributes(table))]
pub fn entity_derive(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(Identifiable, attributes(id))]
pub fn identifiable_derive(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(Timestamps)]
pub fn timestamps_derive(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(AutoMigrate, attributes(table, column, has_many, has_one, belongs_to))]
pub fn auto_migrate_derive(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(Validate, attributes(validate))]
pub fn validate_derive(input: TokenStream) -> TokenStream {
    input
}
