extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Lit, Meta};

#[proc_macro_derive(Signature, attributes(port))]
pub fn signature_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (struct_doc, field_docs) = extract_docs(&input.attrs, &input.data);

    let expanded = quote! {
        impl Signature for #name {
            fn struct_doc() -> &'static str {
                #struct_doc
            }

            fn field_docs() -> std::collections::HashMap<&'static str, &'static str> {
                let mut docs = std::collections::HashMap::new();
                #(#field_docs)*
                docs
            }
        }
    };

    TokenStream::from(expanded)
}

fn extract_docs(attrs: &[Attribute], data: &Data) -> (String, Vec<proc_macro2::TokenStream>) {
    let struct_doc = attrs
        .iter()
        .find_map(|attr| {
            if attr.path.is_ident("doc") {
                match attr.parse_meta() {
                    Ok(Meta::NameValue(meta)) => {
                        if let Lit::Str(lit) = meta.lit {
                            Some(lit.value())
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .unwrap_or_default();

    let field_docs = if let Data::Struct(data) = data {
        if let Fields::Named(fields) = &data.fields {
            fields
                .named
                .iter()
                .map(|field| {
                    let name = &field.ident;
                    let doc = field
                        .attrs
                        .iter()
                        .find_map(|attr| {
                            if attr.path.is_ident("doc") {
                                match attr.parse_meta() {
                                    Ok(Meta::NameValue(meta)) => {
                                        if let Lit::Str(lit) = meta.lit {
                                            Some(lit.value())
                                        } else {
                                            None
                                        }
                                    }
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        })
                        .unwrap_or_default();
                    quote! {
                        docs.insert(stringify!(#name), #doc);
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    (struct_doc, field_docs)
}
