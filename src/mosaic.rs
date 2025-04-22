use dioxus::prelude::*;
use log::Level;

use crate::{mosaic_root::MosaicRoot, prelude::MosaicNode};

#[derive(Clone)]
pub struct MosaicContext {
    pub root_node: Signal<MosaicNode>,
}

#[component]
pub fn Mosaic(root: MosaicNode, children: Element) -> Element {
    let _ = console_log::init_with_level(Level::Debug);

    use_context_provider(|| MosaicContext {
        root_node: Signal::new(root.clone()),
    });

    rsx! {
        div {
            class: "mosaic",
            { children }
            MosaicRoot { }
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MosaicDirection {
    Column,
    Row,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MosaicBranchIndex {
    First,
    Second,
}

#[derive(PartialEq, Clone, Debug)]
pub struct MosaicBranch {
    branch: Vec<MosaicBranchIndex>,
}

impl MosaicBranch {
    pub fn empty() -> MosaicBranch {
        MosaicBranch { branch: vec![] }
    }

    pub fn concat(&self, branch_index: MosaicBranchIndex) -> MosaicBranch {
        let mut new = self.branch.clone();
        new.push(branch_index);

        MosaicBranch { branch: new }
    }
}
