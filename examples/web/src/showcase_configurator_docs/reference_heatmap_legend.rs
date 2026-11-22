use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let range = HeatmapRange::new(
        CivilDate::parse_iso("2024-02-16").unwrap(),
        CivilDate::parse_iso("2025-02-15").unwrap(),
    )
    .unwrap();
    let data = HeatmapData::new(
        (0..range.len())
            .filter(|index| index % 5 != 0)
            .map(|index| (range.date(index).unwrap(), (index % 11) as f64)),
    )
    .unwrap();
    heatmap("reference-heatmap-legend")
        .theme(theme)
        .range(range)
        .data(data)
        .cell_size(10.)
        .gap(2.)
        .radius(2.)
        .colors([
            Color(235, 237, 240, 255),
            Color(155, 233, 168, 255),
            Color(64, 196, 99, 255),
            Color(48, 161, 78, 255),
            Color(33, 110, 57, 255),
        ])
        .with_weekday_labels(true)
        .with_month_labels(true)
        .with_legend(true)
}
