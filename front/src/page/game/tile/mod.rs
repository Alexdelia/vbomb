use dioxus::prelude::*;

mod val;
use val::ValTile;
mod info;
use info::InfoTile;
mod none;
use none::NoneTile;

#[derive(Props, PartialEq)]
pub struct ValTileProps {
    pub v: u8,
    pub open: bool,
}

#[derive(Props, PartialEq)]
pub struct InfoTileProps {
    pub point: u8,
    pub bomb: u8,
}

#[derive(PartialEq)]
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
