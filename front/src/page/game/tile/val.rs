use dioxus::prelude::*;

use super::ValTileProps;

#[allow(non_snake_case)]
pub fn ValTile(cx: Scope<ValTileProps>) -> Element {
    let open = use_state(&cx, || cx.props.open);
    // let tile = match open.get() {
    //     true => rsx!(Open { v: cx.props.v }),
    //     false => rsx!(
    //         section {
    //             onclick: move |_| {
    //                 open.set(true);
    //             },
    //             rsx!(Close {})
    //         }
    //     ),
    // };

    // transform: rotateY(180deg);

    cx.render(rsx!(
        div {
            class: "tile",
            div {
                class: "flip-container",
                Open { v: cx.props.v },
                section {
                    onclick: move |_| {
                        open.set(true);
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
    cx.render(rsx!(
        div {
            class: "close flip-front",
            "Close",
        }
    ))
}
