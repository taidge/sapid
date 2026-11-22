use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let source = r#"{"name":"Sapid","settings":{"enabled":true,"theme":{"color":"blue"}},"members":["Ada","Lin"]}"#;
        let tree = sapid::parse_json_tree(source, sapid::JsonViewLimits::default())
            .expect("valid example JSON");
        let expanded = scope.cage(tree.expanded_at_depth(1));
        sapid::stack()
            .theme(theme.clone())
            .gap(8.)
            .fill((
                sapid::group().theme(theme.clone()).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Expand all")
                        .on_press(move || {
                            expanded.revise(|p| *p = tree.expanded_at_depth(usize::MAX))
                        }),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Collapse all")
                        .on_press(move || expanded.revise(Vec::clear)),
                )),
                sapid::json_viewer("expand-controls")
                    .theme(theme.clone())
                    .source(source)
                    .expanded(expanded)
                    .on_expanded_change(move |next| expanded.revise(|p| *p = next)),
            ))
            .show_in(scope)
    })
}
