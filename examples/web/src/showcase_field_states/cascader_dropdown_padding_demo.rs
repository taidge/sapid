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
        .fill((
            view().class("site-dd-padding-zero").fill(
                sapid::field("cascader-padding-a-field")
                    .theme(theme.clone())
                    .label("No padding")
                    .control(move |field| {
                        sapid::cascader("cascader-padding-a")
                            .placeholder("Pick location")
                            .nodes(vec![
                                sapid::TreeNode::new("eu", "Europe").kind(sapid::TreeKind::Branch),
                                sapid::TreeNode::new("gb", "United Kingdom").parent("eu"),
                                sapid::TreeNode::new("fr", "France").parent("eu"),
                                sapid::TreeNode::new("as", "Asia").kind(sapid::TreeKind::Branch),
                                sapid::TreeNode::new("jp", "Japan").parent("as"),
                                sapid::TreeNode::new("cn", "China").parent("as"),
                            ])
                            .field(field)
                    }),
            ),
            view().class("site-dd-padding-ten").fill(
                sapid::field("cascader-padding-b-field")
                    .theme(theme.clone())
                    .label("10px padding")
                    .control(move |field| {
                        sapid::cascader("cascader-padding-b")
                            .placeholder("Pick location")
                            .nodes(vec![
                                sapid::TreeNode::new("eu", "Europe").kind(sapid::TreeKind::Branch),
                                sapid::TreeNode::new("gb", "United Kingdom").parent("eu"),
                                sapid::TreeNode::new("fr", "France").parent("eu"),
                                sapid::TreeNode::new("as", "Asia").kind(sapid::TreeKind::Branch),
                                sapid::TreeNode::new("jp", "Japan").parent("as"),
                                sapid::TreeNode::new("cn", "China").parent("as"),
                            ])
                            .field(field)
                    }),
            ),
        ))
}
