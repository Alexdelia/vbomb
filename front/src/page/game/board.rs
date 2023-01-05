use dioxus::prelude::*;

use super::tile::{Tile, TileType};

use crate::SIZE;

#[derive(Props, PartialEq)]
struct BoardProps {
    tile: Vec<TileType>,
}

#[allow(non_snake_case)]
pub fn Board(cx: Scope<BoardProps>) -> Element {
    let gtc = "1fr ".repeat(SIZE);

    cx.render(rsx!(
        style { [include_str!("./board.css")] }
        section {
            class: "board",
            grid_template_columns: "{gtc}",
            cx.props.tile.iter().enumerate().map(|(i, t)| {
                rsx!(
                    Tile {
                        idx: i,
                        tile: *t,
                    }
                )
            })
        }
    ))
}
