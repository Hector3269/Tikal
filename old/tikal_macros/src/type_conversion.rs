use quote::quote;
use syn::{Ident, Type};
use crate::utils::extract_option_inner_type;

pub fn generate_to_value(field_name: &Ident, field_type: &Type) -> proc_macro2::TokenStream {
    if let Some(inner_type) = extract_option_inner_type(field_type) {
        let inner_conversion = generate_to_value_for_type(field_name, &inner_type, true);
        return quote! {
            match &self.#field_name {
                Some(val) => #inner_conversion,
                None => tikal::domain::value_objects::Value::Null,
            }
        };
    }

    generate_to_value_for_type(field_name, field_type, false)
}

fn generate_to_value_for_type(
    field_name: &Ident, 
    field_type: &Type,
    is_option_inner: bool
) -> proc_macro2::TokenStream {
    if let Type::Path(type_path) = field_type {
        if let Some(segment) = type_path.path.segments.last() {
            let type_name = segment.ident.to_string();
            
            return match type_name.as_str() {
                "String" => {
                    if is_option_inner {
                        quote! { tikal::domain::value_objects::Value::Text(val.clone()) }
                    } else {
                        quote! { tikal::domain::value_objects::Value::Text(self.#field_name.clone()) }
                    }
                },
                "i64" => {
                    if is_option_inner {
                        quote! { tikal::domain::value_objects::Value::Int(*val) }
                    } else {
                        quote! { tikal::domain::value_objects::Value::Int(self.#field_name) }
                    }
                },
                "i32" | "i16" | "i8" => {
                    if is_option_inner {
                        quote! { tikal::domain::value_objects::Value::Int(*val as i64) }
                    } else {
                        quote! { tikal::domain::value_objects::Value::Int(self.#field_name as i64) }
                    }
                },
                "f64" => {
                    if is_option_inner {
                        quote! { tikal::domain::value_objects::Value::Float(ordered_float::OrderedFloat(*val)) }
                    } else {
                        quote! { tikal::domain::value_objects::Value::Float(ordered_float::OrderedFloat(self.#field_name)) }
                    }
                },
                "f32" => {
                    if is_option_inner {
                        quote! { tikal::domain::value_objects::Value::Float(ordered_float::OrderedFloat(*val as f64)) }
                    } else {
                        quote! { tikal::domain::value_objects::Value::Float(ordered_float::OrderedFloat(self.#field_name as f64)) }
                    }
                },
                "bool" => {
                    if is_option_inner {
                        quote! { tikal::domain::value_objects::Value::Bool(*val) }
                    } else {
                        quote! { tikal::domain::value_objects::Value::Bool(self.#field_name) }
                    }
                },
                "DateTime" => {
                    if is_option_inner {
                        quote! { tikal::domain::value_objects::Value::DateTime(*val) }
                    } else {
                        quote! { tikal::domain::value_objects::Value::DateTime(self.#field_name) }
                    }
                },
                "NaiveDateTime" => {
                    if is_option_inner {
                        quote! { tikal::domain::value_objects::Value::NaiveDateTime(*val) }
                    } else {
                        quote! { tikal::domain::value_objects::Value::NaiveDateTime(self.#field_name) }
                    }
                },
                "Value" if type_path.path.segments.iter().any(|s| s.ident == "serde_json") => {
                    if is_option_inner {
                        quote! { tikal::domain::value_objects::Value::Json(val.clone()) }
                    } else {
                        quote! { tikal::domain::value_objects::Value::Json(self.#field_name.clone()) }
                    }
                },
                "Vec" => {
                    if is_vec_u8(segment) {
                        if is_option_inner {
                            quote! { tikal::domain::value_objects::Value::Binary(val.clone()) }
                        } else {
                            quote! { tikal::domain::value_objects::Value::Binary(self.#field_name.clone()) }
                        }
                    } else {
                        fallback_conversion(field_name, is_option_inner)
                    }
                },
                _ => fallback_conversion(field_name, is_option_inner)
            };
        }
    }
    
    fallback_conversion(field_name, is_option_inner)
}

fn is_vec_u8(segment: &syn::PathSegment) -> bool {
    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
        if let Some(syn::GenericArgument::Type(Type::Path(inner_path))) = args.args.first() {
            if let Some(inner_segment) = inner_path.path.segments.last() {
                return inner_segment.ident == "u8";
            }
        }
    }
    false
}

fn fallback_conversion(field_name: &Ident, is_option_inner: bool) -> proc_macro2::TokenStream {
    if is_option_inner {
        quote! { tikal::domain::value_objects::Value::Text(format!("{:?}", val)) }
    } else {
        quote! { tikal::domain::value_objects::Value::Text(format!("{:?}", self.#field_name)) }
    }
}

pub fn generate_from_value(
    field_name: &Ident,
    field_type: &Type,
    column_name: &str,
    struct_name: &Ident,
) -> proc_macro2::TokenStream {
    let get_value = quote! {
        row.get(#column_name).cloned().ok_or_else(|| {
            tikal::domain::TikalError::db(
                &format!(
                    "Missing column '{}' for field '{}' in struct '{}'",
                    #column_name,
                    stringify!(#field_name),
                    stringify!(#struct_name)
                )
            )
        })?
    };
    if let Some(inner) = extract_option_inner_type(field_type) {
        let inner_expr = generate_from_value_inner(&inner, column_name, field_name);
        return quote! {
            match #get_value {
                tikal::domain::value_objects::Value::Null => None,
                _ => Some(#inner_expr),
            }
        };
    }

    generate_from_value_inner(field_type, column_name, field_name)
}

fn generate_from_value_inner(
    field_type: &Type,
    column_name: &str,
    _field_name: &Ident,
) -> proc_macro2::TokenStream {
    let get_value = quote! {
        row.get(#column_name).cloned().ok_or_else(|| {
            tikal::domain::TikalError::db(
                &format!("Missing column '{}'", #column_name)
            )
        })?
    };

    if let Type::Path(type_path) = field_type {
        if let Some(segment) = type_path.path.segments.last() {
            let type_name = segment.ident.to_string();
            
            return match type_name.as_str() {
                "String" => quote! {
                    match #get_value {
                        tikal::domain::value_objects::Value::Text(s) => s.clone(),
                        _ => return Err(tikal::domain::TikalError::db(
                            &format!("Expected String for column '{}'", #column_name)
                        )),
                    }
                },
                "i64" => quote! {
                    match #get_value {
                        tikal::domain::value_objects::Value::Int(i) => i,
                        _ => return Err(tikal::domain::TikalError::db(
                            &format!("Expected i64 for column '{}'", #column_name)
                        )),
                    }
                },
                "i32" => quote! {
                    match #get_value {
                        tikal::domain::value_objects::Value::Int(i) => i as i32,
                        _ => return Err(tikal::domain::TikalError::db(
                            &format!("Expected i32 for column '{}'", #column_name)
                        )),
                    }
                },
                "bool" => quote! {
                    match #get_value {
                        tikal::domain::value_objects::Value::Bool(b) => b,
                        tikal::domain::value_objects::Value::Int(i) => i != 0,
                        _ => return Err(tikal::domain::TikalError::db(
                            &format!("Expected bool for column '{}'", #column_name)
                        )),
                    }
                },
                "f64" => quote! {
                    match #get_value {
                        tikal::domain::value_objects::Value::Float(f) => f.into_inner(),
                        _ => return Err(tikal::domain::TikalError::db(
                            &format!("Expected f64 for column '{}'", #column_name)
                        )),
                    }
                },
                "f32" => quote! {
                    match #get_value {
                        tikal::domain::value_objects::Value::Float(f) => f.into_inner() as f32,
                        _ => return Err(tikal::domain::TikalError::db(
                            &format!("Expected f32 for column '{}'", #column_name)
                        )),
                    }
                },
                "DateTime" => quote! {
                    match #get_value {
                        tikal::domain::value_objects::Value::DateTime(dt) => dt,
                        _ => return Err(tikal::domain::TikalError::db(
                            &format!("Expected DateTime for column '{}'", #column_name)
                        )),
                    }
                },
                "NaiveDateTime" => quote! {
                    match #get_value {
                        tikal::domain::value_objects::Value::NaiveDateTime(dt) => dt,
                        _ => return Err(tikal::domain::TikalError::db(
                            &format!("Expected NaiveDateTime for column '{}'", #column_name)
                        )),
                    }
                },
                _ => quote! {
                    return Err(tikal::domain::TikalError::db(
                        &format!("Unsupported field type for column '{}'", #column_name)
                    ))
                }
            };
        }
    }
    
    quote! {
        return Err(tikal::domain::TikalError::db(
            &format!("Unsupported field type for column '{}'", #column_name)
        ))
    }
}