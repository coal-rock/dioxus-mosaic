use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct MosaicWindowProps {
    title: String,
    children: Element,
    style: Option<String>,
}

pub fn MosaicWindow(props: MosaicWindowProps) -> Element {
    rsx! {
        div {
            class: "mosaic-window",
            style: props.style.unwrap_or_else(|| String::new()),
            div {
                class: "mosaic-window-toolbar",
                {props.title}
            }

            div {
                class: "mosaic-window-body",
                {props.children}
            }
        }
    }
}
