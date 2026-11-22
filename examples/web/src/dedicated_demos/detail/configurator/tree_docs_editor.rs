use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(vec![
            sapid::TreeNode::new("getting-started", "Getting started")
                .kind(sapid::TreeKind::Branch),
            sapid::TreeNode::new("getting-started--introduction", "Introduction")
                .parent("getting-started"),
            sapid::TreeNode::new("getting-started--installation", "Installation")
                .parent("getting-started"),
            sapid::TreeNode::new("getting-started--quick-start", "Quick start")
                .parent("getting-started"),
            sapid::TreeNode::new("components", "Components").kind(sapid::TreeKind::Branch),
            sapid::TreeNode::new("components--inputs", "Inputs")
                .parent("components")
                .kind(sapid::TreeKind::Branch),
            sapid::TreeNode::new("components--inputs--button", "Button")
                .parent("components--inputs"),
            sapid::TreeNode::new("components--inputs--text-input", "TextInput")
                .parent("components--inputs"),
            sapid::TreeNode::new("components--inputs--select", "Select")
                .parent("components--inputs"),
            sapid::TreeNode::new("components--navigation", "Navigation")
                .parent("components")
                .kind(sapid::TreeKind::Branch),
            sapid::TreeNode::new("components--navigation--tabs", "Tabs")
                .parent("components--navigation"),
            sapid::TreeNode::new("components--navigation--menu", "Menu")
                .parent("components--navigation"),
            sapid::TreeNode::new("hooks", "Hooks").kind(sapid::TreeKind::Branch),
            sapid::TreeNode::new("hooks--use-toggle", "use-toggle").parent("hooks"),
            sapid::TreeNode::new("hooks--use-click-outside", "use-click-outside").parent("hooks"),
        ]);
        let query = scope.cage(String::new());
        let selected = scope.cage(String::new());
        let edit = scope.cage(String::new());
        let next_title = scope.cage(String::new());
        let expanded = scope.cage(std::collections::BTreeSet::from([
            "components".to_owned(),
            "components--inputs".to_owned(),
        ]));
        let visible = scope
            .bond(move || sapid::filter_tree_nodes(&data.get(), &query.get()).unwrap_or_default());
        let ink = theme.clone();
        let content = sapid::tree("docs-navigation-files")
            .theme(theme.clone())
            .name("Documentation navigation")
            .default_selected(std::collections::BTreeSet::from([
                "components--inputs--button".into(),
            ]))
            .file_icons(true)
            .with_lines(true)
            .nodes(visible)
            .expanded(expanded)
            .on_expanded_change(move |next| expanded.revise(|v| *v = next))
            .on_selection_intent(move |key| selected.revise(|v| *v = key))
            .on_drag_drop(move |proposal| {
                if let Ok(next) = sapid::move_tree_node(&data.get(), &proposal) {
                    data.revise(|v| *v = next);
                }
            })
            .render_label(move |node| {
                let key = node.get().key;
                let del = key.clone();
                let rename = key.clone();
                let title_key = key.clone();
                let own = node.get().kind != sapid::TreeKind::Leaf;
                let count = data
                    .get()
                    .iter()
                    .filter(|r| r.parent.as_ref() == Some(&key))
                    .count();
                sapid::group()
                    .theme(ink.clone())
                    .class("site-ref-doc-row")
                    .gap(6.)
                    .fill((
                        text().content(data.map(move |rows| {
                            rows.iter()
                                .find(|r| r.key == title_key)
                                .map(|r| r.label.clone())
                                .unwrap_or_default()
                        })),
                        text().class("site-ref-doc-count").content(if own {
                            count.to_string()
                        } else {
                            String::new()
                        }),
                        sapid::dropdown_menu(format!("doc-actions-{key}"))
                            .theme(ink.clone())
                            .label("Actions")
                            .trigger_content(text().content("⋯"))
                            .item(sapid::menu_item("rename").label("Rename").on_activate(
                                move || {
                                    edit.revise(|v| *v = rename.clone());
                                    next_title.revise(|v| {
                                        *v = data.with(|rows| {
                                            rows.iter()
                                                .find(|r| r.key == rename)
                                                .map(|r| r.label.clone())
                                                .unwrap_or_default()
                                        })
                                    });
                                },
                            ))
                            .item(sapid::menu_item("delete").label("Delete").on_activate(
                                move || {
                                    let prefix = format!("{del}--");
                                    data.revise(|v| {
                                        v.retain(|r| r.key != del && !r.key.starts_with(&prefix))
                                    });
                                },
                            )),
                    ))
            });
        let ink = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .class("site-ref-doc-editor")
            .gap(10.)
            .fill((
                sapid::text_field()
                    .theme(theme.clone())
                    .name("Search docs")
                    .placeholder("Search docs...")
                    .value(query)
                    .on_value_change(move |next| {
                        query.revise(|v| *v = next);
                        expanded.revise(|v| {
                            *v = if query.get().is_empty() {
                                std::collections::BTreeSet::from([
                                    "components".to_owned(),
                                    "components--inputs".to_owned(),
                                ])
                            } else {
                                sapid::expanded_tree_branches(&visible.get()).unwrap_or_default()
                            }
                        });
                    }),
                content,
                when(edit.map(|v| !v.is_empty()), move || {
                    sapid::group().theme(ink.clone()).fill((
                        sapid::text_field()
                            .theme(ink.clone())
                            .name("Page title")
                            .value(next_title)
                            .on_value_change(move |next| next_title.revise(|v| *v = next)),
                        sapid::button()
                            .theme(ink.clone())
                            .label("Save")
                            .on_press(move || {
                                let title = next_title.get();
                                if !title.trim().is_empty() {
                                    data.revise(|v| {
                                        if let Some(row) =
                                            v.iter_mut().find(|r| r.key == edit.get())
                                        {
                                            row.label = title.clone();
                                        }
                                    });
                                    edit.revise(String::clear);
                                }
                            }),
                    ))
                }),
            ))
            .show_in(scope)
    })
}
