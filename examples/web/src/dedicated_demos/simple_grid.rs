use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::responsive_root(640.).content(move |context| {
        let theme = theme.clone();
        widget(move |scope| {
            let tracks = |count| {
                Style::new().grid_columns(glory::style::GridTracks::Tracks(
                    vec![glory::style::GridTrack::Fr(1.); count],
                ))
            };
            let columns = context.styles(sapid::ResponsiveValues::new(
                tracks(1),
                tracks(2),
                tracks(3),
                tracks(3),
                tracks(3),
            ))?;
            sapid::simple_grid("project-cards")
                .theme(theme.clone())
                .columns(3)
                .gap(12.)
                .items(
                    [
                        "Planning",
                        "Design review with the project team",
                        "Delivery",
                    ]
                    .into_iter()
                    .enumerate()
                    .map(|(i, label)| {
                        sapid::grid_item(i.to_string())
                            .style(
                                Style::new()
                                    .grid_column(glory::style::GridPlacement::Auto)
                                    .grid_row(glory::style::GridPlacement::Auto),
                            )
                            .fill(
                                sapid::card()
                                    .theme(theme.clone())
                                    .fill(sapid::text().theme(theme.clone()).content(label)),
                            )
                    }),
                )
                .style(columns)
                .show_in(scope)
        })
    })
}
