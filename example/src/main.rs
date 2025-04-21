use dioxus::prelude::*;
use dioxus_mosaic::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let left = MosaicNode::new(Some(rsx! {}), MosaicDirection::Row, None, None, Some(50.0));
    let left = MosaicNode::to_child_node(left);

    let right_top = MosaicNode::new(Some(rsx! {}), MosaicDirection::Row, None, None, None);
    let right_top = MosaicNode::to_child_node(right_top);

    let right_bot = MosaicNode::new(Some(rsx! {}), MosaicDirection::Row, None, None, None);
    let right_bot = MosaicNode::to_child_node(right_bot);

    let right = MosaicNode::new(
        None,
        MosaicDirection::Column,
        Some(right_top),
        Some(right_bot),
        Some(50.0),
    );
    let right = MosaicNode::to_child_node(right);

    let root = MosaicNode::new(
        None,
        MosaicDirection::Row,
        Some(left),
        Some(right),
        Some(50.0),
    );

    rsx! {
        style {
            r#"
            .mosaic-tile {{
                position: absolute;
                margin: 3px;
                background-color: black;
            }}

            .mosaic-split-line {{
            }}

            .mosaic-split-row {{
                position: absolute;
                z-index: 1;
                touch-action: none;

                margin-left: -3px;
                width: 6px;
                cursor: ew-resize;
            }}

            .mosaic-split-col {{
                position: absolute;
                z-index: 1;
                touch-action: none;
                position: absolute;

                margin-top: -3px;
                height: 6px;
                cursor: ns-resize;
            }}
            "#
        }

        Mosaic {
            root: root
        }
    }
}
