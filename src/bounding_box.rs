use crate::mosaic::MosaicDirection;

#[derive(Clone, PartialEq)]
pub struct BoundingBox {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl BoundingBox {
    pub fn empty() -> BoundingBox {
        BoundingBox {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }

    pub fn as_style(&self) -> String {
        format!(
            "inset: {}% {}% {}% {}%;",
            self.top, self.right, self.bottom, self.left
        )
    }

    pub fn split(
        bounding_box: &BoundingBox,
        relative_split_percentage: f32,
        direction: &MosaicDirection,
    ) -> Split {
        let absolute_percentage = BoundingBox::get_absolute_split_percentage(
            &bounding_box,
            relative_split_percentage,
            &direction,
        );

        match direction {
            MosaicDirection::Column => Split {
                first: BoundingBox {
                    top: bounding_box.top,
                    right: bounding_box.right,
                    bottom: 100.0 - absolute_percentage,
                    left: bounding_box.left,
                },
                second: BoundingBox {
                    top: absolute_percentage,
                    right: bounding_box.right,
                    bottom: bounding_box.bottom,
                    left: bounding_box.left,
                },
            },
            MosaicDirection::Row => Split {
                first: BoundingBox {
                    top: bounding_box.top,
                    right: 100.0 - absolute_percentage,
                    bottom: bounding_box.bottom,
                    left: bounding_box.left,
                },
                second: BoundingBox {
                    top: bounding_box.top,
                    right: bounding_box.right,
                    bottom: bounding_box.bottom,
                    left: absolute_percentage,
                },
            },
        }
    }

    pub fn get_absolute_split_percentage(
        bounding_box: &BoundingBox,
        relative_split_percentage: f32,
        direction: &MosaicDirection,
    ) -> f32 {
        match direction {
            MosaicDirection::Column => {
                let height = 100.0 - bounding_box.top - bounding_box.bottom;
                (height * relative_split_percentage) / 100.0 + bounding_box.top
            }
            MosaicDirection::Row => {
                let width = 100.0 - bounding_box.right - bounding_box.left;
                (width * relative_split_percentage) / 100.0 + bounding_box.left
            }
        }
    }

    pub fn get_relative_split_percentage(
        &self,
        absolute_split_percentage: f32,
        direction: MosaicDirection,
    ) -> f32 {
        todo!()
    }
}

pub struct Split {
    pub first: BoundingBox,
    pub second: BoundingBox,
}

impl Split {}
