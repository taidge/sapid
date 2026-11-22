use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut rows = Vec::with_capacity(2000);
    for branch in 0..20 {
        let id = format!("folder-{branch}");
        rows.push(
            sapid::TreeNode::new(&id, format!("Folder {branch}")).kind(sapid::TreeKind::Branch),
        );
        for child in 0..99 {
            rows.push(
                sapid::TreeNode::new(format!("{id}-{child}"), format!("File {branch}.{child}"))
                    .parent(&id),
            );
        }
    }
    let expanded = sapid::expanded_tree_branches(&rows).expect("2,000 valid entries");
    sapid::tree("virtual-project-tree")
        .theme(theme)
        .class("site-ref-virtual-tree")
        .name("Large project tree")
        .nodes(rows)
        .default_expanded(expanded)
        .file_icons(true)
        .with_lines(true)
        .height(400.)
        .virtual_rows(32., 20)
}
