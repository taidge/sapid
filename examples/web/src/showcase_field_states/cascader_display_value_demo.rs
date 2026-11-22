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
            sapid::field("cascade-display-field")
                .theme(theme.clone())
                .label("Location")
                .control(move |field| {
                    sapid::cascader("cascade-display")
                        .placeholder("Pick location")
                        .nodes(vec![
                            sapid::TreeNode::new("asia", "Asia").kind(sapid::TreeKind::Branch),
                            sapid::TreeNode::new("jp", "Japan")
                                .parent("asia")
                                .kind(sapid::TreeKind::Branch),
                            sapid::TreeNode::new("tokyo", "Tokyo").parent("jp"),
                            sapid::TreeNode::new("osaka", "Osaka").parent("jp"),
                            sapid::TreeNode::new("kyoto", "Kyoto").parent("jp"),
                            sapid::TreeNode::new("kr", "South Korea")
                                .parent("asia")
                                .kind(sapid::TreeKind::Branch),
                            sapid::TreeNode::new("seoul", "Seoul").parent("kr"),
                            sapid::TreeNode::new("busan", "Busan").parent("kr"),
                            sapid::TreeNode::new("europe", "Europe").kind(sapid::TreeKind::Branch),
                            sapid::TreeNode::new("fr", "France")
                                .parent("europe")
                                .kind(sapid::TreeKind::Branch),
                            sapid::TreeNode::new("paris", "Paris").parent("fr"),
                            sapid::TreeNode::new("lyon", "Lyon").parent("fr"),
                            sapid::TreeNode::new("de", "Germany")
                                .kind(sapid::TreeKind::Branch)
                                .parent("europe"),
                            sapid::TreeNode::new("berlin", "Berlin").parent("de"),
                            sapid::TreeNode::new("munich", "Munich").parent("de"),
                            sapid::TreeNode::new("north-america", "North America")
                                .kind(sapid::TreeKind::Branch),
                            sapid::TreeNode::new("us", "United States")
                                .parent("north-america")
                                .kind(sapid::TreeKind::Branch),
                            sapid::TreeNode::new("new-york", "New York").parent("us"),
                            sapid::TreeNode::new("san-francisco", "San Francisco").parent("us"),
                            sapid::TreeNode::new("ca", "Canada")
                                .parent("north-america")
                                .kind(sapid::TreeKind::Branch),
                            sapid::TreeNode::new("toronto", "Toronto").parent("ca"),
                            sapid::TreeNode::new("vancouver", "Vancouver").parent("ca"),
                        ])
                        .path_separator(" › ")
                        .default_value(
                            sapid::TreeChoices::new([sapid::TreePath::new([
                                "asia".into(),
                                "jp".into(),
                                "tokyo".into(),
                            ])
                            .unwrap()])
                            .unwrap(),
                        )
                        .field(field)
                }),
        )
}
