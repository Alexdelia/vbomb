use crate::page::game::tile::{InfoTileProps, TileType, ValTileProps};
use crate::SIZE;

pub fn gen_board() -> Vec<TileType> {
    let mut tile = Vec::with_capacity(SIZE * SIZE);
    let mut rng = rand::thread_rng();

    for x in 0..SIZE {
        for y in 0..SIZE {
            if x == SIZE - 1 && y == SIZE - 1 {
                tile.push(TileType::None);
            } else if x == SIZE - 1 || y == SIZE - 1 {
                tile.push(TileType::Info(InfoTileProps { point: 0, bomb: 0 }));
            } else {
                tile.push(TileType::Val(ValTileProps {
                    v: rng.gen_range(0..6),
                    open: false,
                }));
            }
        }
    }

    for x in 0..SIZE - 1 {
        for y in 0..SIZE - 1 {
            let v = tile[x * SIZE + y].v;
            if v == 0 {
                tile[(x + 1) * SIZE + y].bomb += 1;
                tile[x * SIZE + (y + 1)].bomb += 1;
            } else {
                tile[(x + 1) * SIZE + y].point += v;
                tile[x * SIZE + (y + 1)].point += v;
            }
        }
    }

    tile
}
