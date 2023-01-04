use dioxus::prelude::*;

use super::InfoTileProps;

#[allow(non_snake_case)]
pub fn InfoTile(cx: Scope<InfoTileProps>) -> Element {
    cx.render(rsx!(
        div {
            class: "tile",
            "{cx.props.point} {cx.props.bomb}",
        }
    ))
}
