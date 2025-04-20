use crate::mosaic::MosaicDirection;
use dioxus::prelude::*;
use std::{cell::RefCell, rc::Rc};

pub type MosaicChildNode = Rc<RefCell<MosaicNode>>;

#[derive(PartialEq, Clone)]
pub struct MosaicNode {
    element: Option<Element>,
    direction: MosaicDirection,
    pub first: Option<MosaicChildNode>,
    pub second: Option<MosaicChildNode>,
    split_percentage: Option<f32>,
}

impl MosaicNode {
    pub fn new(
        element: Option<Element>,
        direction: MosaicDirection,
        first: Option<MosaicChildNode>,
        second: Option<MosaicChildNode>,
        split_percentage: Option<f32>,
    ) -> MosaicNode {
        if element.is_some() && (first.is_some() || second.is_some()) {
            panic!("MosaicNode cannot have both an element children")
        }

        if element.is_none() && first.is_none() && second.is_none() {
            panic!("MosaicNode must have either an element or children")
        }

        MosaicNode {
            element,
            direction,
            first,
            second,
            split_percentage,
        }
    }

    pub fn to_child_node(node: MosaicNode) -> MosaicChildNode {
        Rc::new(RefCell::new(node))
    }

    pub fn is_parent(&self) -> bool {
        self.first.is_some() || self.second.is_some()
    }

    pub fn get_split_percentage(&self) -> f32 {
        self.split_percentage.unwrap_or_else(|| 50.0)
    }

    pub fn get_direction(&self) -> &MosaicDirection {
        &self.direction
    }

    pub fn get_element(&self) -> Element {
        match &self.element {
            Some(element) => element.clone(),
            None => rsx! {},
        }
    }
}
