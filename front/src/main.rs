use dioxus::prelude::*;

mod algo;
use algo::gen_board::gen_board;
mod page;
use page::game::board::Board;

pub const SIZE: usize = 6;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let tile = gen_board();

    cx.render(rsx! (
        section {
            style { [include_str!("./style.css")] }
            Board {
                tile: tile,
            }
        }
    ))
}
