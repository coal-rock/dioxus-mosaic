use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;

use crate::bounding_box::BoundingBox;
use crate::mosaic::{MosaicBranch, MosaicBranchIndex, MosaicDirection};
use crate::mosaic_node::{MosaicChildNode, MosaicNode};
use crate::mosaic_split::MosaicSplit;

#[component]
pub fn MosaicRoot(root: MosaicNode) -> Element {
    let root = Rc::new(RefCell::new(root));
    let children = render_recursively(Some(root), BoundingBox::empty(), MosaicBranch::empty());

    rsx! {
        div {
            class: "mosaic-root",
            {children.iter()}
        }
    }
}

fn render_recursively(
    node: Option<MosaicChildNode>,
    bounding_box: BoundingBox,
    path: MosaicBranch,
) -> Vec<Element> {
    let node_ref = match &node {
        Some(node) => node.borrow(),
        None => return vec![],
    };

    match node_ref.is_parent() {
        true => {
            let split_percentage = node_ref.get_split_percentage();

            let split =
                BoundingBox::split(&bounding_box, split_percentage, node_ref.get_direction());

            let mut elements = Vec::new();

            elements.extend(render_recursively(
                node_ref.first.clone(),
                split.first,
                path.concat(MosaicBranchIndex::First),
            ));

            elements.extend(render_split(
                node_ref.get_direction(),
                bounding_box,
                split_percentage,
                &path,
            ));

            elements.extend(render_recursively(
                node_ref.second.clone(),
                split.second,
                path.concat(MosaicBranchIndex::Second),
            ));

            elements
        }
        false => {
            vec![rsx! {
                div {
                    class: "mosaic-tile",
                    style: bounding_box.as_style(),

                    {node_ref.get_element()}
                }
            }]
        }
    }
}

fn render_split(
    direction: &MosaicDirection,
    bounding_box: BoundingBox,
    split_percentage: f32,
    path: &MosaicBranch,
) -> Vec<Element> {
    vec![rsx! {
        MosaicSplit {
            direction: direction.clone(),
            bounding_box: bounding_box,
            split_percentage: split_percentage,
        }
    }]
}
