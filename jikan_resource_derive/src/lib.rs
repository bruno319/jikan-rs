extern crate proc_macro;

use proc_macro::TokenStream;
use std::iter::repeat;

use syn;
use syn::{Data, DeriveInput, Lit, Meta, Variant};

use quote::quote;

#[proc_macro_derive(Resource, attributes(uri_prefix, rename_uri))]
pub fn resource_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input)
        .expect("Unable to implement Resource trait through derive macro");

    let uri_prefix = find_prefix_attr(&ast);
    impl_resource_macro(&ast, uri_prefix)
}

fn find_prefix_attr(ast: &DeriveInput) -> Option<String> {
    let mut prefix = None;
    for attr in ast.attrs.iter() {
        let meta = attr.parse_meta().unwrap();
        match meta {
            Meta::NameValue(meta) if meta.path.is_ident("uri_prefix") => {
                match meta.lit {
                    Lit::Str(lit) => {
                        prefix = Some(lit.value());
                    }
                    _ => panic!("Prefix attribute should be a string")
                }
            }
            _ => {}
        }
    }
    prefix
}

fn impl_resource_macro(ast: &syn::DeriveInput, prefix: Option<String>) -> TokenStream {
    let prefix = prefix.unwrap_or(String::from(""));

    let gen = match &ast.data {
        Data::Enum(ref data_enum) => {
            let enum_name = &ast.ident;
            let repeated_enum_name = repeat(&ast.ident);
            let variant_names = data_enum.variants
                .iter()
                .map(|variant| &variant.ident);
            let uri_variants = data_enum.variants
                .iter()
                .map(|variant| mount_uri(&prefix, variant));
            quote! {
                impl Resource for #enum_name {
                    fn uri(&self) -> String {
                        match self {
                            #(#repeated_enum_name::#variant_names => #uri_variants.to_string(),)*
                        }
                    }
                }
            }
        }
        _ => panic!("#[derive(Resource)] is only available for enums")
    };
    gen.into()
}

fn mount_uri(prefix: &String, variant: &Variant) -> String {
    for attr in variant.attrs.iter() {
        let meta = attr.parse_meta().unwrap();
        match meta {
            Meta::NameValue(meta) if meta.path.is_ident("rename_uri") => {
                match meta.lit {
                    Lit::Str(lit) => {
                        return format!("{}{}", prefix, lit.value());
                    }
                    _ => panic!("Prefix attribute should be a string")
                }
            }
            _ => {}
        }
    }
    format!("{}{}", prefix, to_snake_case(&mut variant.ident.to_string()))
}

fn to_snake_case(string: &mut String) -> String {
    if !string.is_empty() {
        string.replace_range(0..1, &string.as_str()[0..1].to_lowercase());
        return string.chars()
            .map(|c| match c {
                'A'..='Z' => format!("_{}", c.to_lowercase()),
                _ => c.to_string()
            })
            .collect::<Vec<String>>()
            .join("")
    }
    String::from("")
}