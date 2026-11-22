use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = theme.clone();
    let field = move |key: &'static str, label: &'static str, feedback: &'static str| {
        let appearance = Style::new().border_color(palette.danger);
        sapid::field(key)
            .theme(palette.clone())
            .label(label)
            .style(Style::new().width(Length::Percent(100.)))
            .invalid(true)
            .error(feedback)
            .control(move |field| {
                sapid::tree_select(key)
                    .nodes(vec![
                        sapid::TreeNode::new("frontend", "Frontend").kind(sapid::TreeKind::Branch),
                        sapid::TreeNode::new("react", "React").parent("frontend"),
                        sapid::TreeNode::new("vue", "Vue").parent("frontend"),
                    ])
                    .field(field)
                    .style(appearance.clone())
                    .placeholder(label)
            })
    };
    sapid::stack()
        .class("site-field-state-example")
        .theme(theme)
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(field("tree_select_error-boolean", "Boolean error", ""))
        .fill(field(
            "tree_select_error-message",
            "With error message",
            "Invalid value",
        ))
}
