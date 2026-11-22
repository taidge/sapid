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
            view().class("site-dd-position").fill(
                sapid::field("tree-select-position-field")
                    .theme(theme.clone())
                    .label("Location")
                    .control(move |field| {
                        sapid::tree_select("tree-select-position")
                            .placeholder("Pick location")
                            .nodes(vec![
                                sapid::TreeNode::new("eu", "Europe").kind(sapid::TreeKind::Branch),
                                sapid::TreeNode::new("gb", "United Kingdom").parent("eu"),
                                sapid::TreeNode::new("fr", "France").parent("eu"),
                                sapid::TreeNode::new("as", "Asia").kind(sapid::TreeKind::Branch),
                                sapid::TreeNode::new("jp", "Japan").parent("as"),
                                sapid::TreeNode::new("cn", "China").parent("as"),
                            ])
                            .popup_placement(glory::render::AnchorPlacement::new(
                                glory::render::AnchorSide::Top,
                                glory::render::AnchorAlign::Start,
                            ))
                            .field(field)
                    }),
            ),
        )
}
