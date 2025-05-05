use dioxus::prelude::*;

use crate::bounding_box::BoundingBox;

#[component]
pub fn MosaicTile(bounding_box: BoundingBox, children: Element) -> Element {
    rsx! {
        div {
            class: "mosaic-tile",
            style: bounding_box.as_style(),

            {children}
        }
    }
}
