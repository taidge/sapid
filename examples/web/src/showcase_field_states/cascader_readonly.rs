use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("cascader_readonly-example")
        .theme(theme.clone())
        .class("site-field-state-example")
        .label("Location")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .readonly(true)
        .control(move |field| {
            sapid::cascader("cascader_readonly-example")
                .nodes(vec![
                    sapid::TreeNode::new("frontend", "Frontend").kind(sapid::TreeKind::Branch),
                    sapid::TreeNode::new("react", "React").parent("frontend"),
                    sapid::TreeNode::new("vue", "Vue").parent("frontend"),
                ])
                .field(field)
                .placeholder("Location")
        })
}
