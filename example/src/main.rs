use dioxus::prelude::*;
use dioxus_mosaic::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn Pink() -> Element {
    rsx! {
        div {
            style: "background-color: pink; width: 100%; height: 100%;"
        }
    }
}

#[component]
fn Red() -> Element {
    rsx! {
        div {
            style: "background-color: red; width: 100%; height: 100%;"
        }
    }
}

#[component]
fn Black() -> Element {
    rsx! {
        div {
            style: "background-color: black; width: 100%; height: 100%;"
        }
    }
}

#[component]
fn Green() -> Element {
    rsx! {
        div {
            style: "background-color: green; width: 100%; height: 100%;"
        }
    }
}

#[component]
fn App() -> Element {
    let mut root = use_signal(|| MosaicNode::new_root(rsx! {Pink{}}));

    rsx! {
        button {
            style: "margin-left: 60px; width: 50px; height: 50px; position: absolute; z-index: 9999",
            onclick: move |_| {
                root.write()
                    .add_child_in_order(MosaicDirection::Column, &rsx! {Red{}});
            },
            "col"
        }

        button {
            style: "width: 50px; height: 50px; position: absolute; z-index: 9999",
            onclick: move |_| {
                root.write()
                    .add_child_in_order(MosaicDirection::Row, &rsx! {Pink{}});
            }
            "row"
        }

        style {
            r#"
            .mosaic-tile {{
                position: absolute;
                margin: 3px;
                background-color: black;
            }}

            .mosaic-split-line {{
            }}

            .mosaic-split-row {{
                position: absolute;
                z-index: 1;
                touch-action: none;

                margin-left: -3px;
                width: 6px;
                cursor: ew-resize;
            }}

            .mosaic-split-col {{
                position: absolute;
                z-index: 1;
                touch-action: none;
                position: absolute;

                margin-top: -3px;
                height: 6px;
                cursor: ns-resize;
            }}
            "#
        }

        Mosaic {
            root: root()
        }
    }
}
