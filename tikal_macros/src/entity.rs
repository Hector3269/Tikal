use heck::ToSnakeCase;
use proc_macro_error::abort;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, Type};
use crate::utils::{extract_table_name, extract_primary_key, extract_column_name, extract_option_inner_type, get_type_mapping};
use crate::type_conversion::generate_to_value;

fn map_type_to_column_type(ty: &Type) -> proc_macro2::TokenStream {
    if let Some(inner) = extract_option_inner_type(ty) {
        return map_type_to_column_type(&inner);
    }

    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let type_name = segment.ident.to_string();
            if type_name == "Vec" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(Type::Path(inner_path))) = args.args.first() {
                        if let Some(inner_segment) = inner_path.path.segments.last() {
                            if inner_segment.ident == "u8" {
                                return quote! { tikal::infrastructure::schema::types::ColumnType::Binary };
                            }
                        }
                    }
                }
                return quote! { tikal::infrastructure::schema::types::ColumnType::Text };
            }
            let mapping = get_type_mapping(&type_name);
            let column_type_ident = syn::Ident::new(mapping.column_type, proc_macro2::Span::call_site());
            return quote! { tikal::infrastructure::schema::types::ColumnType::#column_type_ident };
        }
    }
    quote! { tikal::infrastructure::schema::types::ColumnType::Text }
}

pub fn expand_entity_derive(input: DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;
    let table_name = extract_table_name(&input);
    let primary_key = extract_primary_key(&input);

    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => abort!(input, "Entity only supports structs with named fields"),
    };

    let mut insertions = Vec::new();
    let mut column_defs = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let column_name = extract_column_name(field)
            .unwrap_or_else(|| field_name.to_string().to_snake_case());

        let value_expr = generate_to_value(field_name, &field.ty);

        insertions.push(quote! {
            values.insert(#column_name.to_string(), #value_expr);
        });

        let column_type = map_type_to_column_type(&field.ty);
        let nullable = extract_option_inner_type(&field.ty).is_some();
        let primary_key = field_name.to_string() == primary_key;
        let auto_increment = primary_key && matches!(field.ty, Type::Path(ref p) if p.path.segments.last().map(|s| s.ident == "i64").unwrap_or(false));

        column_defs.push(quote! {
            tikal::infrastructure::schema::types::ColumnDefinition {
                name: #column_name.to_string(),
                column_type: #column_type,
                nullable: #nullable,
                primary_key: #primary_key,
                auto_increment: #auto_increment,
                default_value: None,
                unique: false,
            }
        });
    }

    quote! {
        impl tikal::domain::model::Entity for #struct_name {
            fn table_name() -> &'static str {
                #table_name
            }

            fn primary_key() -> &'static str {
                #primary_key
            }

            fn to_values(&self) -> std::collections::HashMap<String, tikal::domain::value_objects::Value> {
                let mut values = std::collections::HashMap::new();
                #(#insertions)*
                values
            }

            fn table_definition() -> tikal::infrastructure::schema::types::TableDefinition {
                tikal::infrastructure::schema::types::TableDefinition {
                    name: #table_name.to_string(),
                    columns: vec![#(#column_defs),*],
                    indexes: vec![],
                }
            }

            fn generate_create_table_sql(driver: &str) -> String {
                use tikal::infrastructure::schema::generators::DdlGenerator;
                tikal::infrastructure::schema::generators::UnifiedDdlGenerator::from_driver(driver)
                    .unwrap()
                    .generate_create_table(&Self::table_definition())
            }

            fn find() -> tikal::domain::query::builder::QueryBuilder<Self> {
                tikal::domain::query::builder::QueryBuilder::new()
            }
        }
    }
}