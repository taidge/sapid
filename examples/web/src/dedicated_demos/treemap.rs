use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut palette = controls::chart_palette(&theme);
    palette.series[..4].copy_from_slice(&[
        Color(59, 91, 219, 255),
        Color(224, 49, 49, 255),
        Color(9, 146, 104, 255),
        Color(52, 58, 64, 255),
    ]);
    palette.grid = theme.surface;
    treemap("treemap-overview")
        .theme(theme)
        .name("Treemap chart")
        .chart_theme(palette)
        .data(PlotData::Hierarchy(
            [
                ("usa", "USA", 400.),
                ("india", "India", 300.),
                ("japan", "Japan", 100.),
                ("other", "Other", 200.),
            ]
            .into_iter()
            .map(|(k, label, v)| HierarchyDatum {
                key: k.into(),
                label: label.into(),
                weight: Some(v),
                parent: None,
            })
            .collect(),
        ))
        .frame(ChartFrame {
            width: 684.,
            height: 300.,
        })
        .default_options(PlotOptions::default())
        .style(Style::new().width(Length::Percent(100.)))
        .legend(false)
        .navigation(false)
        .data_table(false)
}
