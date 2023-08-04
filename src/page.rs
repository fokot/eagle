#![allow(non_snake_case)]
use crate::{Url, Version};
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
struct EnvProps {
    env: Url,
    result: Version,
}

// FIXME cloning too much here
fn Env(cx: Scope<EnvProps>) -> Element {
    cx.render(rsx! {
        li {
            div {
                style: "font-weight: bold; padding-top: 10px;",
                cx.props.env.clone(),
            }
            match &cx.props.result {
                Ok(env) => rsx!( div { env.text.clone() }),
                Err(e) => rsx!( div { format!("error: {}", e) } ),
            },
        }
    })
}

#[derive(PartialEq, Props)]
struct BodyProps {
    envs: Vec<(Url, Version)>,
}

fn Body(cx: Scope<BodyProps>) -> Element {
    cx.render(rsx! {
        h1 {
            "Environment versions:",
        },
        ul {
            cx.props.envs.iter().map(|entry| rsx!(Env{ env: entry.0.clone(), result: entry.1.clone() })),
        }
    })
}

pub fn render(data: Vec<(Url, Version)>) -> String {
    dioxus_ssr::render_lazy(rsx! {
      Body { envs: data }
    })
}
