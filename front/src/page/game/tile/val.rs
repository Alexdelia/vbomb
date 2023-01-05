use dioxus::prelude::*;

use super::ValTileProps;

#[allow(non_snake_case)]
pub fn ValTile(cx: Scope<ValTileProps>) -> Element {
    let class_name = use_state(&cx, || if cx.props.open { "coin" } else { "" });

    cx.render(rsx!(
        div {
            class: "tile",
            div {
                class: "flip-container {class_name}",
                Open { v: cx.props.v },
                div {
                    onclick: move |_| {
                        class_name.set("coin");
                    },
                    Close {}
                }
            }
        }
    ))
}

#[derive(Props, PartialEq)]
struct OpenProps {
    v: u8,
}

#[allow(non_snake_case)]
fn Open(cx: Scope<OpenProps>) -> Element {
    cx.render(rsx!(
        div {
            class: "open flip-back",
            "{cx.props.v}",
        }
    ))
}

#[allow(non_snake_case)]
fn Close(cx: Scope) -> Element {
    cx.render(rsx!(img {
        class: "close flip-front",
        width: "100%",
        height: "100%",
        src: "/assets/tile_back.svg",
        alt: "B",
    }))
}
