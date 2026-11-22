use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("tree_select_readonly-example")
        .theme(theme.clone())
        .class("site-field-state-example")
        .label("Your favorite item")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .readonly(true)
        .control(move |field| {
            sapid::tree_select("tree_select_readonly-example")
                .nodes(vec![
                    sapid::TreeNode::new("frontend", "Frontend").kind(sapid::TreeKind::Branch),
                    sapid::TreeNode::new("react", "React").parent("frontend"),
                    sapid::TreeNode::new("vue", "Vue").parent("frontend"),
                ])
                .field(field)
                .placeholder("Your favorite item")
        })
}
