use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn NoneTile(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "tile",
            "None",
        }
    ))
}
