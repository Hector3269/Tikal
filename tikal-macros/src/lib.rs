use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Lit, parse_macro_input};

#[proc_macro_derive(ModelMapping, attributes(table, column))]
pub fn model_mapping_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let attrs = &input.attrs;
    // Parse table name from #[table(name = "...")]
    let mut table_name = None;
    for attr in attrs {
        if attr.path().is_ident("table") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let value = meta.value()?;
                    let lit: Lit = value.parse()?;
                    if let Lit::Str(lit_str) = lit {
                        table_name = Some(lit_str.value());
                    }
                }
                Ok(())
            })
            .unwrap();
        }
    }
    let table_name =
        table_name.expect("Table name must be specified with #[table(name = \"...\")]");
    let fields = if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            &fields_named.named
        } else {
            panic!("ModelMapping only supports structs with named fields");
        }
    } else {
        panic!("ModelMapping only supports structs");
    };
    let mut columns = vec![];
    let mut primary_key = None;
    for field in fields {
        let field_name = field.ident.as_ref().unwrap().to_string();
        let mut column_name = field_name.clone();
        let mut is_primary = false;
        for attr in &field.attrs {
            if attr.path().is_ident("column") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        let value = meta.value()?;
                        let lit: Lit = value.parse()?;
                        if let Lit::Str(lit_str) = lit {
                            column_name = lit_str.value();
                        }
                    } else if meta.path.is_ident("primary_key") {
                        is_primary = true;
                    }
                    Ok(())
                })
                .unwrap();
            }
        }
        columns.push((field_name, column_name.clone()));
        if is_primary {
            if primary_key.is_some() {
                panic!("Only one primary key allowed");
            }
            primary_key = Some(column_name);
        }
    }
    let primary_key = primary_key.expect("No primary key defined");
    let columns_array = columns
        .iter()
        .map(|(rust, db)| quote! { (#rust, #db) })
        .collect::<Vec<_>>();
    let expanded = quote! {
        impl #struct_name {
            pub const TABLE_NAME: &'static str = #table_name;
            pub fn columns() -> &'static [(&'static str, &'static str)] {
                &[ #(#columns_array),* ]
            }
            pub fn primary_key() -> &'static str {
                #primary_key
            }
        }
    };
    TokenStream::from(expanded)
}
