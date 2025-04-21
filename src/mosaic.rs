use dioxus::prelude::*;
use log::Level;
use log::info;

use crate::{mosaic_root::MosaicRoot, prelude::MosaicNode};

#[component]
pub fn Mosaic(root: MosaicNode) -> Element {
    console_log::init_with_level(Level::Debug);
    info!("{:#?}", root);

    rsx! {
        div {
            class: "mosaic",

            MosaicRoot {
                root: root
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MosaicDirection {
    Column,
    Row,
}

#[derive(Clone)]
pub enum MosaicBranchIndex {
    First,
    Second,
}

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
