extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AllFieldNamesAsString, attributes(StructFieldAsString))]
pub fn all_field_name_as_str_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let fields =
        if let syn::Data::Struct(
            syn::DataStruct {
                fields: syn::Fields::Named(
                    syn::FieldsNamed { named, .. }), ..
            }) = input.data
        {
            named
        } else {
            panic!("Only works on structs with named fields")
        };

        let field_parts = fields.iter().map(|f| {
            let ident = &f.ident;
            if f.attrs.iter().any(|attr| attr.path().is_ident("StructFieldAsString")) {
                // Field has the `#[helper]` attribute, assume it implements the trait
                quote! {
                    field_parts.push(self.#ident.get_all_field_names_as_string());
                }
            } else {
                // Field does not have the `#[helper]` attribute, use the field name as string
                quote! {
                    let field_str = stringify!(#ident).to_string();
                    field_parts.push(field_str);
                }
            }
        });

    let expanded = quote! {
        impl #name {
            fn get_all_field_names_as_string(&self) -> String {
                let mut field_parts: Vec<String> = Vec::new();
                #(#field_parts)*
                field_parts.join(", ")
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(AllValuesAsString, attributes(StructFieldAsString, NumericField, BooleanField))]
pub fn all_values_as_str_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let fields =
        if let syn::Data::Struct(
            syn::DataStruct {
                fields: syn::Fields::Named(
                    syn::FieldsNamed { named, .. }), ..
            }) = input.data
        {
            named
        } else {
            panic!("Only works on structs with named fields")
        };

        let field_parts = fields.iter().map(|f| {
            let ident = &f.ident;
            if f.attrs.iter().any(|attr| attr.path().is_ident("StructFieldAsString")) {
                // Field has the `#[helper]` attribute, assume it implements the trait
                quote! {
                    field_parts.push(self.#ident.get_all_values_as_string());
                }
            } else if f.attrs.iter().any(|attr| attr.path().is_ident("NumericField")) {
                quote! {
                    let field_str = format!("{}", &self.#ident);
                    field_parts.push(field_str);
                }
            } else if f.attrs.iter().any(|attr| attr.path().is_ident("BooleanField")) {
                quote! {
                    let field_str = if self.#ident { "1".into() } else { "0".into() };
                    field_parts.push(field_str);
                }
            }else {
                // Field does not have the `#[helper]` attribute, use the field name as string
                quote! {
                    let field_str = format!("'{}'", &self.#ident);
                    field_parts.push(field_str);
                }
            }
        });

    let expanded = quote! {
        impl #name {
            fn get_all_values_as_string(&self) -> String {
                let mut field_parts: Vec<String> = Vec::new();
                #(#field_parts)*
                field_parts.join(", ")
            }
        }
    };

    TokenStream::from(expanded)
}