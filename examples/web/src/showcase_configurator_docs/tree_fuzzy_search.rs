use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let query = scope.cage(String::new());
        let rows = vec![
            sapid::TreeNode::new("src", "src").kind(sapid::TreeKind::Branch),
            sapid::TreeNode::new("src--components", "components")
                .parent("src")
                .kind(sapid::TreeKind::Branch),
            sapid::TreeNode::new("src--components--Accordion-tsx", "Accordion.tsx")
                .parent("src--components"),
            sapid::TreeNode::new("src--components--Tree-tsx", "Tree.tsx").parent("src--components"),
            sapid::TreeNode::new("src--components--Button-tsx", "Button.tsx")
                .parent("src--components"),
            sapid::TreeNode::new("src--components--Input-tsx", "Input.tsx")
                .parent("src--components"),
            sapid::TreeNode::new("src--hooks", "hooks")
                .parent("src")
                .kind(sapid::TreeKind::Branch),
            sapid::TreeNode::new("src--hooks--use-debounce-ts", "use-debounce.ts")
                .parent("src--hooks"),
            sapid::TreeNode::new("src--hooks--use-media-query-ts", "use-media-query.ts")
                .parent("src--hooks"),
            sapid::TreeNode::new("public", "public").kind(sapid::TreeKind::Branch),
            sapid::TreeNode::new("public--favicon-ico", "favicon.ico").parent("public"),
            sapid::TreeNode::new("public--logo-svg", "logo.svg").parent("public"),
            sapid::TreeNode::new("package-json", "package.json"),
            sapid::TreeNode::new("tsconfig-json", "tsconfig.json"),
        ];
        let expanded = scope.cage(std::collections::BTreeSet::<String>::new());
        let visible = scope.bond(move || {
            let text = query.get().trim().to_lowercase();
            if text.is_empty() {
                return rows.clone();
            }
            let mut keep = std::collections::BTreeSet::new();
            for row in &rows {
                let label = row.label.to_lowercase();
                let mut chars = label.chars();
                if text
                    .chars()
                    .all(|ch| chars.any(|candidate| candidate == ch))
                {
                    let mut key = row.key.as_str();
                    loop {
                        keep.insert(key.to_owned());
                        let Some((ancestor, _)) = key.rsplit_once("--") else {
                            break;
                        };
                        key = ancestor;
                    }
                }
            }
            rows.iter()
                .filter(|r| keep.contains(&r.key))
                .cloned()
                .collect()
        });
        sapid::stack()
            .theme(theme.clone())
            .class("site-ref-file-tree")
            .gap(10.)
            .fill((
                sapid::text_field()
                    .theme(theme.clone())
                    .name("Fuzzy search")
                    .placeholder("Fuzzy search...")
                    .value(query)
                    .on_value_change(move |s| {
                        query.revise(|v| *v = s);
                        expanded.revise(|v| {
                            *v = if query.get().trim().is_empty() {
                                Default::default()
                            } else {
                                sapid::expanded_tree_branches(&visible.get()).unwrap_or_default()
                            }
                        });
                    }),
                sapid::tree("fuzzy-project-tree")
                    .theme(theme.clone())
                    .name("Search results")
                    .file_icons(true)
                    .with_lines(true)
                    .nodes(visible)
                    .expanded(expanded)
                    .on_expanded_change(move |v| expanded.revise(|s| *s = v)),
            ))
            .show_in(scope)
    })
}
