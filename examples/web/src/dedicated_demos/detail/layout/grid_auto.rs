use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::grid("auto-columns")
        .theme(theme.clone())
        .column_tracks([
            glory::style::GridTrack::Fr(1.),
            glory::style::GridTrack::Fr(2.),
            glory::style::GridTrack::Fr(1.),
        ])
        .gap(16.)
        .items((1..=3).map(|i| {
            sapid::grid_item(i.to_string()).fill(controls::grid_demo_cell(i.to_string(), &theme))
        }))
}
