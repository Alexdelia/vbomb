use rand::Rng;

use super::cast::cast;
use crate::page::game::tile::{InfoTileProps, TileType, ValTileProps};
use crate::SIZE;

pub fn gen_board() -> Vec<TileType> {
    let mut board: Vec<TileType> = Vec::with_capacity(SIZE * SIZE);
    let mut rng = rand::thread_rng();

    for x in 0..SIZE {
        for y in 0..SIZE {
            if x == SIZE - 1 && y == SIZE - 1 {
                board.push(TileType::None);
            } else if x == SIZE - 1 || y == SIZE - 1 {
                board.push(TileType::Info(InfoTileProps::default()));
            } else {
                board.push(TileType::Val(ValTileProps::new(rng.gen_range(0..6))));
            }
        }
    }

    let in_size = SIZE - 1;
    for x in 0..in_size {
        for y in 0..in_size {
            let v = cast!(board[x * SIZE + y], TileType::Val).v;

            if v == 0 {
                cast!(board[x * SIZE + in_size], TileType::Info).bomb += 1;
                cast!(board[in_size * SIZE + y], TileType::Info).bomb += 1;
            } else {
                cast!(board[x * SIZE + in_size], TileType::Info).point += 1;
                cast!(board[in_size * SIZE + y], TileType::Info).point += 1;
            }
        }
    }

    board
}
