pub use dsp_macros::Signature;

mod context;
mod error;
mod llm;
mod modules;

pub use self::{
    context::Context,
    error::Error,
    modules::{Chain, Module},
};

pub trait Signature
where
    Self: Default,
{
    type Inputs;
    type Outputs;
}

#[macro_export]
macro_rules! chain {
    ( $( $from:ident -> $to:ident ),* $(,)?) => {{
        println!("Creating chain from {} to {}", stringify!($($from)*), stringify!($($to)*));

        let mut chain = ::dsp::Chain::<$($from)*, $($to)*>::new();

        chain
    }};
}

#[macro_export]
macro_rules! params {
    ( $( $key:ident => $value:expr ),* $(,)? ) => {{
        let mut params = ::std::collections::HashMap::new();
        $(
            params.insert(stringify!($key), $value);
        )*
        params
    }}
}

#[macro_export]
macro_rules! request {
    ($module:ident, { $($key:ident => $value:expr),* $(,)? }) => {
        async {
            let context = ::dsp::Context::new();
            let mut params = ::dsp::params!($($key => $value)*);

            let schema = params
                .iter()
                .map(|(k, _)| format!("<{k}>A value.</{k}>", k=k))
                .collect::<Vec<String>>().join("\n");
            let preprompt = format!("Please provide the result in the following format:\n{}", schema);

            let prompt = params
                .iter()
                .map(|(k, v)| format!("<{k}>{v}</{k}>", k=k, v=v))
                .collect::<Vec<String>>().join("\n");

            let result = context.llm.complete(&[preprompt, prompt].join("\n\n")).await?;
            dbg!(&result);

            Ok(result)
        }
    };
}
