use crate::mosaic::MosaicDirection;
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
}
