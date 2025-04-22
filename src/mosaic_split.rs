use dioxus::prelude::*;
use log::info;

use crate::bounding_box::BoundingBox;
use crate::mosaic::{MosaicBranch, MosaicDirection};

#[component]
pub fn MosaicSplit(
    direction: MosaicDirection,
    bounding_box: BoundingBox,
    split_percentage: f32,
    path: MosaicBranch,
) -> Element {
    rsx! {
        div {
            class: match direction {
                MosaicDirection::Column => "mosaic-split-col",
                MosaicDirection::Row => "mosaic-split-row",
            },
            style: compute_style(
                bounding_box,
                direction,
                split_percentage,
            ),

            onmousedown: move |event| {
                event.prevent_default();
                info!("down");
                info!("{:#?}", path);
            },
            onmouseup: move |event| {
                event.prevent_default();
                info!("up");
            },
            onmousemove: move |event| {
                event.prevent_default();
                // e = e || window.event;
                // e.preventDefault();
                // // calculate the new cursor position:
                // pos1 = pos3 - e.clientX;
                // pos2 = pos4 - e.clientY;
                // pos3 = e.clientX;
                // pos4 = e.clientY;
                // // set the element's new position:
                // elmnt.style.top = (elmnt.offsetTop - pos2) + "px";
                // elmnt.style.left = (elmnt.offsetLeft - pos1) + "px";
                info!("moving");

                info!("{:#?}", event.page_coordinates());
            },
            div {
                class: "mosaic-split-line"
            }
        }
    }
}

fn compute_style(
    bounding_box: BoundingBox,
    direction: MosaicDirection,
    split_percentage: f32,
) -> String {
    let position_style = match direction {
        MosaicDirection::Column => "top: ",
        MosaicDirection::Row => "left: ",
    };

    let absolute_percentage =
        BoundingBox::get_absolute_split_percentage(&bounding_box, split_percentage, &direction);

    let mut style = String::new();
    style.push_str(&bounding_box.as_style());
    style.push_str("\n");

    style.push_str(position_style);
    style.push_str(&absolute_percentage.to_string());
    style.push_str("%;");

    style
}
