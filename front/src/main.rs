use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        section {
            style { [include_str!("./style.css")] }
            div {
                class: "dbg",
                span {
                    class: "dbg",
                    "Hello, world!"
                }
                span {
                    class: "dbg",
                    "42"
                }
            }
        }
    ))
}
