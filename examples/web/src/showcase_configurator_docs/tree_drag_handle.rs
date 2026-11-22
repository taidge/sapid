use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(vec![
            sapid::TreeNode::new("pages", "Pages").kind(sapid::TreeKind::Branch),
            sapid::TreeNode::new("pages--index-tsx", "index.tsx").parent("pages"),
            sapid::TreeNode::new("pages--about-tsx", "about.tsx").parent("pages"),
            sapid::TreeNode::new("components", "Components").kind(sapid::TreeKind::Branch),
            sapid::TreeNode::new("components--Header-tsx", "Header.tsx").parent("components"),
            sapid::TreeNode::new("components--Footer-tsx", "Footer.tsx").parent("components"),
            sapid::TreeNode::new("package-json", "package.json"),
        ]);
        sapid::tree("file-row-handles")
            .theme(theme.clone())
            .class("site-ref-file-tree")
            .name("Reorder from the grab handles")
            .file_icons(true)
            .with_lines(true)
            .nodes(data)
            .drag_handle(true)
            .on_drag_drop(move |change| {
                if let Ok(next) = sapid::move_tree_node(&data.get(), &change) {
                    data.revise(|v| *v = next);
                }
            })
            .show_in(scope)
    })
}
