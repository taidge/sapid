use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "tree-select-props",
        "Input label",
        "Input description",
        move |field, style| {
            sapid::tree_select("tree-select-props-control")
                .field(field)
                .style(style)
                .placeholder("Choose a value")
                .nodes(vec![
                    sapid::TreeNode::new("frontend", "Frontend").kind(sapid::TreeKind::Branch),
                    sapid::TreeNode::new("react", "React").parent("frontend"),
                    sapid::TreeNode::new("vue", "Vue").parent("frontend"),
                ])
        },
    )
}
