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
    heatmap("reference-heatmap-change-colors")
        .theme(theme)
        .range(range)
        .data(data)
        .cell_size(10.)
        .gap(2.)
        .radius(2.)
        .colors([
            Color(235, 237, 240, 255),
            Color(255, 169, 77, 255),
            Color(253, 126, 20, 255),
            Color(247, 103, 7, 255),
            Color(217, 72, 15, 255),
        ])
        .with_weekday_labels(false)
        .with_month_labels(false)
        .with_legend(false)
}
