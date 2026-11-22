use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("tree_select_success-accepted")
        .theme(theme.clone())
        .label("Tree Select")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .success(true)
        .class("site-field-state-example site-valid-field")
        .description("Looks good!")
        .order([
            sapid::FieldSlot::Label,
            sapid::FieldSlot::Control,
            sapid::FieldSlot::Description,
            sapid::FieldSlot::Error,
        ])
        .control(move |field| {
            sapid::tree_select("tree_select_success-accepted")
                .nodes(vec![
                    sapid::TreeNode::new("frontend", "Frontend").kind(sapid::TreeKind::Branch),
                    sapid::TreeNode::new("react", "React").parent("frontend"),
                    sapid::TreeNode::new("vue", "Vue").parent("frontend"),
                ])
                .field(field)
                .style(Style::new().border_color(theme.success))
                .placeholder("Tree Select")
        })
}
