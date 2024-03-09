extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};



#[proc_macro_derive(Signature, attributes(signature, input, output))]
pub fn signature_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the implementation
    let name = &input.ident;
    match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let inputs: Vec<_> = fields
                    .named
                    .iter()
                    .filter(|f| f.attrs.iter().any(|a| a.path.is_ident("input")))
                    .map(|f| &f.ty)
                    .collect();
                let outputs: Vec<_> = fields
                    .named
                    .iter()
                    .filter(|f| f.attrs.iter().any(|a| a.path.is_ident("output")))
                    .map(|f| &f.ty)
                    .collect();

                let gen = quote! {
                    impl ::dsp::Signature for #name {
                        type Inputs = (#(#inputs),*);
                        type Outputs = (#(#outputs),*);
                    }
                };

                gen.into()
            }
            _ => panic!("Signature macro only supports structs with named fields."),
        },
        _ => panic!("Signature macro only supports structs."),
    }
}
