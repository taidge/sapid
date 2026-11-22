use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(vec![
            sapid::TreeNode::new("pages", "Pages").kind(sapid::TreeKind::Branch),
            sapid::TreeNode::new("pages--index-tsx", "index.tsx").parent("pages"),
            sapid::TreeNode::new("pages--about-tsx", "about.tsx").parent("pages"),
            sapid::TreeNode::new("components", "Components (locked)").kind(sapid::TreeKind::Branch),
            sapid::TreeNode::new("components--Header-tsx", "Header.tsx").parent("components"),
            sapid::TreeNode::new("components--Footer-tsx", "Footer.tsx").parent("components"),
            sapid::TreeNode::new("package-json", "package.json"),
        ]);
        sapid::tree("restricted-project-tree")
            .theme(theme.clone())
            .name("Reorder project files")
            .class("site-ref-file-tree")
            .file_icons(true)
            .with_lines(true)
            .nodes(data)
            .allow_drop(|p| {
                !p.dragged.starts_with("components")
                    && !p.target.starts_with("components--")
                    && !(p.target == "components" && p.position == sapid::TreeDropPosition::Inside)
            })
            .on_drag_drop(move |change| {
                if let Ok(next) = sapid::move_tree_node(&data.get(), &change) {
                    data.revise(|v| *v = next);
                }
            })
            .show_in(scope)
    })
}
