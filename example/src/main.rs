use dioxus::prelude::*;
use dioxus_mosaic::{mosaic::MosaicContext, prelude::*};

fn main() {
    dioxus::launch(App);
}

#[component]
fn PinkWindow() -> Element {
    rsx! {
        div {
            style: "background-color: pink; width: 100%; height: 100%;"
        }
    }
}

#[component]
fn RedWindow() -> Element {
    let mut counter = use_signal(|| 0);

    rsx! {
        div {
            style: "background-color: red; width: 100%; height: 100%;",
            button {
                style: "width: 50px; height: 50px; z-index: 9999",
                onclick: move |_| {
                    *counter.write() += 1;
                },
                {counter.to_string()}
            }
        }
    }
}

#[component]
fn SplitColButton() -> Element {
    let mut root = use_context::<MosaicContext>().root_node;

    rsx! {
        button {
            style: "margin-left: 60px; width: 50px; height: 50px; position: absolute; z-index: 9999",
            onclick: move |_| {
                root.write()
                    .add_child_in_order(MosaicDirection::Column, &rsx! {
                        MosaicWindow {
                            title: "hello",
                            children: rsx! { RedWindow {} },
                        }
                    });
            },
            "col"
        }
    }
}

#[component]
fn SplitRowButton() -> Element {
    let mut root = use_context::<MosaicContext>().root_node;

    rsx! {
        button {
            style: "width: 50px; height: 50px; position: absolute; z-index: 9999",
            onclick: move |_| {
                root.write()
                    .add_child_in_order(MosaicDirection::Row, &rsx! {
                        MosaicWindow {
                            title: "hello",
                            children: rsx! { PinkWindow {} },
                        }
                    });
            },
            "row"
        }
    }
}

#[component]
fn App() -> Element {
    let root = use_signal(|| {
        MosaicNode::new_root(rsx! {
            MosaicWindow {
                title: "root",
                children: rsx! { RedWindow {} },
                style: "width: 100%; height: 100%; border: 2px solid black; border-radius: 2px; overflow: hidden; margin: -2px;"
            }
        })
    });

    rsx! {
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
            
            .mosaic-window {{
                width: 100%; 
                height: 100%; 
                border: 2px solid black; 
                border-radius: 2px; 
                margin: -2px;
                
                position: relative;
                display: flex;
                flex-direction: column;
                overflow: hidden;
            }}

            .mosaic-window-toolbar {{
                display: flex;
                justify-content: space-between;
                align-items: center;
                flex-shrink: 0;
                height: 30px;
                background: white;
            }}

            .mosaic-window-body {{
                width: 100%;
                height: 100%
            }}
            "#
        }

        Mosaic {
            root: root(),
            SplitColButton {

            }

            SplitRowButton {

            }
        }
    }
}
