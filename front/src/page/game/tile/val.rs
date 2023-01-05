use dioxus::prelude::*;

use super::ValTileProps;

#[allow(non_snake_case)]
pub fn ValTile(cx: Scope<ValTileProps>) -> Element {
    let open = use_state(&cx, || cx.props.open);
    let flip = use_state(&cx, || if *open.get() { "coin" } else { "coin" });
    // let transform = use_state(&cx, || "".to_string());

    // let tile = match open.get() {
    //     true => rsx!(Open { v: cx.props.v }),
    //     false => rsx!(
    //         div {
    //             onclick: move |_| {
    //                 open.set(true);
    //                 transform.set("rotateY(180deg)".to_string());
    //             },
    //             Close {}
    //         }
    //     ),
    // };

    cx.render(rsx!(
        div {
            class: "tile",
            div {
                class: "flip-container {flip}",
                // transform: "{transform}",
                Open { v: cx.props.v },
                div {
                    onclick: move |_| {
                        open.set(true);
                        flip.set("coin");
                        // transform.set("rotateY(180deg)".to_string());
                    },
                    Close {}
                }
                // tile
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
