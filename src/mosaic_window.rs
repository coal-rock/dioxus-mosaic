use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct MosaicWindowProps {
    title: String,
    children: Element,
}

pub fn MosaicWindow(props: MosaicWindowProps) -> Element {
    rsx! {
        {props.children}
    }
}
