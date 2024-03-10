use lazy_static::lazy_static;
use tera::Tera;

pub use dsp_macros::Signature;

mod context;
mod error;
mod llm;
mod modules;

pub use self::{context::Context, error::Error, llm::*, modules::*};

pub trait Signature
where
    Self: Default,
{
    fn struct_doc() -> &'static str;
    fn field_docs() -> std::collections::HashMap<&'static str, &'static str>;
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();

        tera.add_raw_template(
            "default_prompt",
            include_str!("templates/default_prompt.jinja2"),
        )
        .expect("Failed to compile template");

        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
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


            let mut render_context = ::tera::Context::new();

            render_context.insert("prompt", "this is the prompt");
            render_context.insert("json_response_schema", "this is the expected json schema");

            for (k, v) in params.iter() {
                render_context.insert(k.to_string(), v);
            }

            let prompt = $crate::TEMPLATES.render("default_prompt", &render_context).unwrap();
            let result = context.llm.complete(&prompt).await?;
            dbg!(&result);

            Ok(result)
        }
    };
}
