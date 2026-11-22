use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::grid("column-align")
        .theme(theme.clone())
        .columns(3)
        .gap(16.)
        .items(
            ["Start", "Center", "End"]
                .into_iter()
                .enumerate()
                .map(|(i, label)| {
                    sapid::grid_item(label).fill(
                        view()
                            .style(Style {
                                justify: Some([Justify::Start, Justify::Center, Justify::End][i]),
                                background: Some(theme.accent_soft),
                                ..Style::new()
                                    .display(Display::Flex)
                                    .flex_direction(FlexDirection::Column)
                                    .min_height(Length::Px(150.))
                            })
                            .fill(
                                text()
                                    .style(Style::new().padding(glory::style::Edges::all(12.)))
                                    .content(label),
                            ),
                    )
                }),
        )
}
