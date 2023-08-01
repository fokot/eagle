use crate::VersionInfo;
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Props)]
struct EnvProps {
    env: String,
    result: Result<VersionInfo, String>,
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
    envs: HashMap<String, Result<VersionInfo, String>>,
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

pub fn render(data: HashMap<String, Result<VersionInfo, String>>) -> String {
    dioxus_ssr::render_lazy(rsx! {
      Body { envs: data }
    })
}
