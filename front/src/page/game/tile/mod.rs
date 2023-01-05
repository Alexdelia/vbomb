use dioxus::prelude::*;

mod val;
use val::ValTile;
mod info;
use info::InfoTile;
mod none;
use none::NoneTile;

pub type Val = u8;

#[derive(Props, PartialEq, Clone, Copy)]
pub struct ValTileProps {
    pub v: Val,
    pub open: bool,
}

impl Default for ValTileProps {
    fn default() -> Self {
        Self { v: 0, open: false }
    }
}

impl ValTileProps {
    pub fn new(v: Val) -> Self {
        Self {
            v,
            ..Default::default()
        }
    }
}

#[derive(Props, PartialEq, Clone, Copy, Default)]
pub struct InfoTileProps {
    pub point: Val,
    pub bomb: u8,
}

#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Val(ValTileProps),
    Info(InfoTileProps),
    None,
}

#[derive(Props, PartialEq)]
pub struct TileProps {
    idx: usize,
    tile: TileType,
}

#[allow(non_snake_case)]
pub fn Tile(cx: Scope<TileProps>) -> Element {
    let tile = match &cx.props.tile {
        TileType::Val(v) => rsx!(ValTile {
            v: v.v,
            open: v.open
        }),
        TileType::Info(i) => rsx!(InfoTile {
            point: i.point,
            bomb: i.bomb
        }),
        TileType::None => rsx!(NoneTile {}),
    };

    cx.render(rsx!(
        style { [include_str!("./tile.css")] }
        tile
    ))
}
