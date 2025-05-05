use dioxus::prelude::*;
use web_sys::window;

use crate::bounding_box::BoundingBox;
use crate::mosaic::{MosaicBranch, MosaicContext, MosaicDirection};

#[component]
pub fn MosaicSplit(
    direction: MosaicDirection,
    bounding_box: BoundingBox,
    split_percentage: f32,
    path: MosaicBranch,
) -> Element {
    let mut root = use_context::<MosaicContext>().root_node;
    let mut mouse_down = use_signal(|| false);

    let path = use_signal(|| path.clone());

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
            draggable: true,

            onmousedown: move |event| {
                *mouse_down.write() = true
            },
            onmouseup: move |event| {
                *mouse_down.write() = false;
            },
            ondragstart: move |event| {
                event.prevent_default();
            },
            onmouseleave: move |event| {
                if *mouse_down.read() == false {
                    return;
                }

                let window = web_sys::window().unwrap();
                let current_position = event.client_coordinates();

                match direction {
                    MosaicDirection::Column => {
                        let window_height = window.inner_height().unwrap().as_f64().unwrap();
                        root.write().resize(path(), current_position.y as f32, window_height as f32);
                    },
                    MosaicDirection::Row => {
                        let window_width = window.inner_width().unwrap().as_f64().unwrap();
                        root.write().resize(path(), current_position.x as f32, window_width as f32);
                    },
                }
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
