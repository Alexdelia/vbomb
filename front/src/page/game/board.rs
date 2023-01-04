use dioxus::prelude::*;

use super::tile::{Tile, TileType, ValTileProps};

use crate::SIZE;

#[allow(non_snake_case)]
pub fn Board(cx: Scope) -> Element {
    let mut tile: Vec<(usize, u8)> = Vec::with_capacity(SIZE * SIZE);
    for i in 0..SIZE * SIZE {
        tile.push((i, (i % SIZE) as u8));
    }

    let gtc = "1fr ".repeat(SIZE);

    cx.render(rsx!(
        style { [include_str!("./board.css")] }
        section {
            class: "board",
            grid_template_columns: "{gtc}",
            tile.iter().map(|t| {
                rsx!(
                    Tile {
                        tile: TileType::Val(ValTileProps {
                            i: t.0,
                            v: t.1,
                            open: false,
                        })
                    }
                )
            })
        }
    ))
}
