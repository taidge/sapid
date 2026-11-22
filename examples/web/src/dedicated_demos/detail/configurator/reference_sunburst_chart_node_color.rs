use glory::prelude::*;
use sapid::charts::*;
fn sample_data() -> PlotData {
    PlotData::Hierarchy(
        [
            ("Sales", 500.),
            ("Marketing", 300.),
            ("Engineering", 400.),
            ("Support", 200.),
        ]
        .map(|(name, weight)| HierarchyDatum {
            key: name.into(),
            label: name.into(),
            parent: None,
            weight: Some(weight),
        })
        .into(),
    )
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut palette = controls::chart_palette(&theme);
    palette.series[..4].copy_from_slice(&[
        Color(121, 80, 242, 255),
        Color(190, 75, 219, 255),
        Color(230, 73, 128, 255),
        Color(250, 82, 82, 255),
    ]);
    palette.grid = theme.surface;
    sunburst_chart("reference-sunburst-chart-node-color")
        .theme(theme)
        .name("Sunburst chart")
        .chart_theme(palette)
        .data(sample_data())
        .frame(ChartFrame {
            width: 320.,
            height: 320.,
        })
        .default_options(PlotOptions {
            counterclockwise: true,
            sunburst_inner_ratio: Some(1. / 3.),
            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(300.))
                .max_width(Limit::Percent(100.)),
        )
        .legend(false)
        .navigation(false)
        .data_table(false)
}
