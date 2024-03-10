extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Lit, Meta};

#[proc_macro_derive(Signature, attributes(port))]
pub fn signature_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let instructions = input
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

    let fields = if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            fields
                .named
                .into_iter()
                .filter_map(|field| {
                    let doc = field.attrs.iter().find_map(|attr| {
                        if attr.path.is_ident("doc") {
                            match attr.parse_meta() {
                                Ok(Meta::NameValue(meta)) => {
                                    if let lit @ Lit::Str(_) = meta.lit {
                                        Some(lit)
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    });
                    let direction = field.attrs.iter().find_map(|attr| {
                        if attr.path.is_ident("port") {
                            match attr.parse_meta() {
                                Ok(Meta::Path(path)) if path.is_ident("input") => {
                                    Some(quote! {::dsp::Direction::Input})
                                }
                                Ok(Meta::Path(path)) if path.is_ident("output") => {
                                    Some(quote! {::dsp::Direction::Outnput})
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    });

                    field.ident.zip(doc).zip(direction)
                })
                .map(|((name, doc), direction)| {
                    quote! {
                        fields.push(Ref {
                            direction: #direction,
                            kind: stringify!(#name),
                            field: stringify!(#name),
                            description: #doc
                        })
                    }
                })
                .collect()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let expanded = quote! {
        impl Signature for #name {
            fn instructions() -> &'static str {
                #instructions
            }

            fn fields() -> Vec<::dsp::Ref> {
                let mut fields = vec![];
                #(#fields)*
                fields
            }

            fn inputs() -> Vec<::dsp::Ref> {
                Self::fields().into_iter().filter(|f| f.direction == ::dsp::Direction::Input).collect()
            }

            fn outputs() -> Vec<::dsp::Ref> {
                Self::fields().into_iter().filter(|f| f.direction == ::dsp::Direction::Output).collect()
            }
        }
    };

    TokenStream::from(expanded)
}
