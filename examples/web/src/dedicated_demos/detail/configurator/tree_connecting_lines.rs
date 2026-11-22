use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let rows = vec![
        sapid::TreeNode::new("src", "src").kind(sapid::TreeKind::Branch),
        sapid::TreeNode::new("src--components", "components")
            .parent("src")
            .kind(sapid::TreeKind::Branch),
        sapid::TreeNode::new("src--components--Accordion-tsx", "Accordion.tsx")
            .parent("src--components"),
        sapid::TreeNode::new("src--components--Tree-tsx", "Tree.tsx").parent("src--components"),
        sapid::TreeNode::new("src--components--Button-tsx", "Button.tsx").parent("src--components"),
        sapid::TreeNode::new("node_modules", "node_modules").kind(sapid::TreeKind::Branch),
        sapid::TreeNode::new("node_modules--react", "react")
            .parent("node_modules")
            .kind(sapid::TreeKind::Branch),
        sapid::TreeNode::new("node_modules--react--index-d-ts", "index.d.ts")
            .parent("node_modules--react"),
        sapid::TreeNode::new("node_modules--react--package-json", "package.json")
            .parent("node_modules--react"),
        sapid::TreeNode::new("node_modules--framework", "@sapid")
            .parent("node_modules")
            .kind(sapid::TreeKind::Branch),
        sapid::TreeNode::new("node_modules--framework--core", "core")
            .parent("node_modules--framework")
            .kind(sapid::TreeKind::Branch),
        sapid::TreeNode::new("node_modules--framework--core--index-d-ts", "index.d.ts")
            .parent("node_modules--framework--core"),
        sapid::TreeNode::new(
            "node_modules--framework--core--package-json",
            "package.json",
        )
        .parent("node_modules--framework--core"),
        sapid::TreeNode::new("node_modules--framework--hooks", "hooks")
            .parent("node_modules--framework")
            .kind(sapid::TreeKind::Branch),
        sapid::TreeNode::new("node_modules--framework--hooks--index-d-ts", "index.d.ts")
            .parent("node_modules--framework--hooks"),
        sapid::TreeNode::new(
            "node_modules--framework--hooks--package-json",
            "package.json",
        )
        .parent("node_modules--framework--hooks"),
        sapid::TreeNode::new("node_modules--framework--form", "form")
            .parent("node_modules--framework")
            .kind(sapid::TreeKind::Branch),
        sapid::TreeNode::new("node_modules--framework--form--index-d-ts", "index.d.ts")
            .parent("node_modules--framework--form"),
        sapid::TreeNode::new(
            "node_modules--framework--form--package-json",
            "package.json",
        )
        .parent("node_modules--framework--form"),
        sapid::TreeNode::new("package-json", "package.json"),
        sapid::TreeNode::new("tsconfig-json", "tsconfig.json"),
    ];
    let expanded = sapid::expanded_tree_branches(&rows).expect("valid project tree");
    sapid::tree("project-tree-lines")
        .theme(theme)
        .name("Project files")
        .with_lines(true)
        .file_icons(true)
        .nodes(rows)
        .default_expanded(expanded)
}
