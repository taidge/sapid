use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::grid("content-column")
        .theme(theme.clone())
        .column_tracks([
            glory::style::GridTrack::Auto,
            glory::style::GridTrack::Percent(50.),
            glory::style::GridTrack::Fr(1.),
        ])
        .gap(16.)
        .items(
            [("content", "fit content"), ("half", "2")]
                .into_iter()
                .map(|(key, label)| {
                    sapid::grid_item(key).fill(controls::grid_demo_cell(label.to_owned(), &theme))
                }),
        )
}
