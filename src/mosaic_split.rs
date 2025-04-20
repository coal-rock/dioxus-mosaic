use dioxus::prelude::*;

use crate::bounding_box::BoundingBox;
use crate::mosaic::MosaicDirection;

#[component]
pub fn MosaicSplit(
    direction: MosaicDirection,
    bounding_box: BoundingBox,
    split_percentage: f32,
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
                split_percentage
            ),

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
