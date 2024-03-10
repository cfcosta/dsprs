use lazy_static::lazy_static;
use tera::Tera;

pub use dsp_macros::Signature;

mod context;
mod error;
mod llm;
mod modules;

pub use self::{context::Context, error::Error, llm::*, modules::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Input,
    Output,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ref {
    pub direction: Direction,
    pub kind: &'static str,
    pub field: &'static str,
    pub description: Option<&'static str>,
}

pub trait Signature
where
    Self: Default,
{
    fn instructions() -> &'static str;
    fn fields() -> Vec<Ref>;
    fn inputs() -> Vec<Ref>;
    fn outputs() -> Vec<Ref>;
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
    ($($start:ident.$start_output:ident -> $end:ident.$end_input:ident),+ $(,)?) => {{
        let mut chain = ::dsp::ChainBuilder::new();

        $(
            let from = ::dsp::Ref {
                direction: ::dsp::Direction::Input,
                kind: stringify!($start),
                field: stringify!($start_output),
                description: Some(<$start>::instructions())
            };


            let to = ::dsp::Ref {
                direction: ::dsp::Direction::Output,
                kind: stringify!($end),
                field: stringify!($end_input),
                description: Some(<$end>::instructions())
            };

            chain.add_link::<$start, $end>(from, to);
        )+

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
