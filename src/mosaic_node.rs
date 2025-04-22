use crate::{
    bounding_box::BoundingBox,
    mosaic::{MosaicBranch, MosaicBranchIndex, MosaicDirection},
    mosaic_split::MosaicSplit,
};
use dioxus::prelude::*;

pub type MosaicChildNode = Box<MosaicNode>;

#[derive(PartialEq, Clone, Debug)]
pub enum MosaicNode {
    Element(Element),
    Children {
        first: MosaicChildNode,
        second: Option<MosaicChildNode>,
        direction: MosaicDirection,
        split_percentage: f32,
    },
}

impl MosaicNode {
    pub fn new_root(element: Element) -> MosaicNode {
        MosaicNode::Element(element)
    }

    pub fn add_child_in_order(&mut self, direction: MosaicDirection, element: &Element) {
        let mut node = self;

        loop {
            match node {
                MosaicNode::Element(inner_element) => {
                    *node = MosaicNode::Children {
                        first: Box::new(MosaicNode::Element(inner_element.clone())),
                        second: Some(Box::new(MosaicNode::Element(element.clone()))),
                        direction,
                        split_percentage: 50.0,
                    };

                    return;
                }
                MosaicNode::Children { second, .. } => match second {
                    Some(child_node) => {
                        node = child_node.as_mut();
                    }
                    None => {
                        *second = Some(Box::new(MosaicNode::Element(element.clone())));
                        return;
                    }
                },
            }
        }
    }

    pub fn to_child_node(node: MosaicNode) -> MosaicChildNode {
        Box::new(node)
    }

    pub fn is_parent(&self) -> bool {
        match self {
            MosaicNode::Element(_) => false,
            MosaicNode::Children { .. } => true,
        }
    }

    pub fn get_split_percentage(&self) -> Option<f32> {
        match self {
            MosaicNode::Element(_) => None,
            MosaicNode::Children {
                split_percentage, ..
            } => Some(*split_percentage),
        }
    }

    pub fn get_direction(&self) -> Option<MosaicDirection> {
        match self {
            MosaicNode::Element(_) => None,
            MosaicNode::Children { direction, .. } => Some(*direction),
        }
    }

    pub fn get_element(&self) -> Option<Element> {
        match self {
            MosaicNode::Element(element) => Some(element.clone()),
            MosaicNode::Children { .. } => None,
        }
    }

    pub fn render(&self, bounding_box: BoundingBox, path: MosaicBranch) -> Vec<Element> {
        match self {
            MosaicNode::Element(element) => {
                vec![rsx! {
                    div {
                        class: "mosaic-tile",
                        style: bounding_box.as_style(),

                        {element.clone()}
                    }
                }]
            }
            MosaicNode::Children { first, second, .. } => {
                let split_percentage = self.get_split_percentage().unwrap();
                let direction = self.get_direction().unwrap();

                let split = BoundingBox::split(&bounding_box, split_percentage, &direction);

                let mut elements = Vec::new();

                elements.extend(first.render(split.first, path.concat(MosaicBranchIndex::First)));

                let split_elements = vec![rsx! {
                    MosaicSplit {
                        direction: direction.clone(),
                        bounding_box: bounding_box,
                        split_percentage: split_percentage,
                        path: path.clone()
                    }
                }];

                let second_elements = match second {
                    Some(second) => {
                        second.render(split.second, path.concat(MosaicBranchIndex::Second))
                    }
                    None => vec![],
                };

                elements.extend(split_elements);
                elements.extend(second_elements);

                elements
            }
        }
    }
}
