use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            sapid::field("cascader-one-column-field")
                .theme(theme.clone())
                .label("Location")
                .control(move |field| {
                    sapid::cascader("cascader-one-column")
                        .placeholder("Pick location")
                        .nodes(vec![
                            sapid::TreeNode::new("eu", "Europe").kind(sapid::TreeKind::Branch),
                            sapid::TreeNode::new("gb", "United Kingdom").parent("eu"),
                            sapid::TreeNode::new("fr", "France").parent("eu"),
                            sapid::TreeNode::new("as", "Asia").kind(sapid::TreeKind::Branch),
                            sapid::TreeNode::new("jp", "Japan").parent("as"),
                            sapid::TreeNode::new("cn", "China").parent("as"),
                        ])
                        .with_columns(false)
                        .field(field)
                }),
        )
}
