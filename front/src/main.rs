use dioxus::prelude::*;

mod page;
use page::game::board::Board;

pub const SIZE: usize = 6;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        section {
            style { [include_str!("./style.css")] }
            Board {
            }
        }
    ))
}
