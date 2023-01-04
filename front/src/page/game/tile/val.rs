use dioxus::prelude::*;

use super::ValTileProps;

#[allow(non_snake_case)]
pub fn ValTile(cx: Scope<ValTileProps>) -> Element {
    let open = use_state(&cx, || cx.props.open);
    let tile = match open.get() {
        true => rsx!(Open {}),
        false => rsx!(
            section {
                onclick: move |_| {
                    open.set(true);
                },
                rsx!(Close {})
            }
        ),
    };

    cx.render(rsx!(
        div {
            class: "tile",
            "{cx.props.v} {cx.props.i} {cx.props.open}",
            tile
        }
    ))
}

#[allow(non_snake_case)]
fn Open(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "open",
            "Open",
        }
    ))
}

#[allow(non_snake_case)]
fn Close(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "close",
            "Close",
        }
    ))
}
