use heck::ToSnakeCase;
use syn::{Field, Lit, DeriveInput};

pub fn extract_column_name(field: &Field) -> Option<String> {
    for attr in &field.attrs {
        if attr.path().is_ident("column_name") {
            if let Ok(meta) = attr.meta.require_name_value() {
                if let syn::Expr::Lit(lit) = &meta.value {
                    if let Lit::Str(lit_str) = &lit.lit {
                        return Some(lit_str.value());
                    }
                }
            }
        }
    }
    None
}

pub fn extract_table_name(input: &DeriveInput) -> String {
    for attr in &input.attrs {
        if attr.path().is_ident("table_name") {
            if let Ok(meta) = attr.meta.require_name_value() {
                if let syn::Expr::Lit(lit) = &meta.value {
                    if let Lit::Str(lit_str) = &lit.lit {
                        return lit_str.value();
                    }
                }
            }
        }
    }

    let struct_name = input.ident.to_string().to_snake_case();
    pluralize(&struct_name)
}

pub fn extract_primary_key(input: &DeriveInput) -> String {
    for attr in &input.attrs {
        if attr.path().is_ident("primary_key") {
            if let Ok(meta) = attr.meta.require_name_value() {
                if let syn::Expr::Lit(lit) = &meta.value {
                    if let Lit::Str(lit_str) = &lit.lit {
                        return lit_str.value();
                    }
                }
            }
        }
    }
    "id".to_string()
}

pub fn extract_option_inner_type(option_type: &syn::Type) -> Option<syn::Type> {
    if let syn::Type::Path(type_path) = option_type {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                        return Some(inner.clone());
                    }
                }
            }
        }
    }
    None
}

pub fn pluralize(word: &str) -> String {
    if word.is_empty() {
        return word.to_string();
    }

    if word.ends_with('y') && !word.ends_with("ay") && !word.ends_with("ey")
        && !word.ends_with("iy") && !word.ends_with("oy") && !word.ends_with("uy") {
        format!("{}ies", &word[..word.len() - 1])
    } else if word.ends_with("ch") || word.ends_with("sh") || word.ends_with('s')
        || word.ends_with('x') || word.ends_with('z') {
        format!("{}es", word)
    } else if word.ends_with("fe") {
        format!("{}ves", &word[..word.len() - 2])
    } else if word.ends_with('f') {
        format!("{}ves", &word[..word.len() - 1])
    } else {
        format!("{}s", word)
    }
}

#[derive(Debug)]
pub struct TypeMapping {
    pub column_type: &'static str,
}

pub fn get_type_mapping(type_name: &str) -> TypeMapping {
    match type_name {
        "String" => TypeMapping {
            column_type: "Text",
        },
        "i64" => TypeMapping {
            column_type: "BigInt",
        },
        "i32" | "i16" | "i8" => TypeMapping {
            column_type: "Int",
        },
        "f64" | "f32" => TypeMapping {
            column_type: "Float",
        },
        "bool" => TypeMapping {
            column_type: "Bool",
        },
        "DateTime" => TypeMapping {
            column_type: "DateTime",
        },
        "NaiveDateTime" => TypeMapping {
            column_type: "NaiveDateTime",
        },
        "Value" => TypeMapping {
            column_type: "Json",
        },
        _ => TypeMapping {
            column_type: "Text",
        },
    }
}

