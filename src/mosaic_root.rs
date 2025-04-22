use dioxus::prelude::*;

use crate::bounding_box::BoundingBox;
use crate::mosaic::{MosaicBranch, MosaicContext};

#[component]
pub fn MosaicRoot() -> Element {
    let root = use_context::<MosaicContext>().root_node;

    let children = root
        .read()
        .render(BoundingBox::empty(), MosaicBranch::empty());

    rsx! {
        div {
            class: "mosaic-root",
            {children.iter()}
        }
    }
}
