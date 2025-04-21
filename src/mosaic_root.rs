use dioxus::prelude::*;
use log::{info, warn};

use crate::bounding_box::BoundingBox;
use crate::mosaic::{MosaicBranch, MosaicBranchIndex, MosaicDirection};
use crate::mosaic_node::{MosaicChildNode, MosaicNode};
use crate::mosaic_split::MosaicSplit;

#[component]
pub fn MosaicRoot(root: MosaicNode) -> Element {
    let children = render_recursively(
        Some(&mut Box::new(root)),
        BoundingBox::empty(),
        MosaicBranch::empty(),
    );

    rsx! {
        div {
            class: "mosaic-root",
            {children.iter()}
        }
    }
}

fn render_recursively(
    node: Option<&mut MosaicChildNode>,
    bounding_box: BoundingBox,
    path: MosaicBranch,
) -> Vec<Element> {
    info!("hi");
    let node = match node {
        Some(node) => node,
        None => return vec![],
    };

    match &**node {
        MosaicNode::Element(element) => {
            vec![rsx! {
                div {
                    class: "mosaic-tile",
                    style: bounding_box.as_style(),

                    {element.clone()}
                }
            }]
        }
        MosaicNode::Children {
            first,
            second,
            direction,
            split_percentage,
        } => {
            let split_percentage = node.get_split_percentage().unwrap();
            let direction = node.get_direction().unwrap();

            let split = BoundingBox::split(&bounding_box, split_percentage, &direction);

            let mut elements = Vec::new();

            elements.extend(render_recursively(
                Some(&mut first.clone()),
                split.first,
                path.concat(MosaicBranchIndex::First),
            ));

            let split_elements = render_split(
                &node.get_direction().unwrap(),
                bounding_box,
                split_percentage,
                &path,
            );

            let second_elements = render_recursively(
                second.clone().as_mut(),
                split.second,
                path.concat(MosaicBranchIndex::Second),
            );

            if second_elements.len() >= 0 {
                elements.extend(split_elements);
                elements.extend(second_elements);
            }

            elements
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
