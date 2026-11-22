use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = controls::chart_palette(&theme);
    radar_chart("radar-chart-overview")
        .theme(theme)
        .name("Product sales")
        .chart_theme(palette)
        .data(PlotData::values([
            ValueDatum::new("group0", "Apples", 120.0),
            ValueDatum::new("group1", "Oranges", 98.0),
            ValueDatum::new("group2", "Tomatoes", 86.0),
            ValueDatum::new("group3", "Grapes", 99.0),
            ValueDatum::new("group4", "Bananas", 85.0),
            ValueDatum::new("group5", "Lemons", 65.0),
        ]))
        .frame(ChartFrame {
            width: 320.,
            height: 320.,
        })
        .default_options(PlotOptions {
            radar_parts: Some(RadarParts {
                grid: true,
                angles: true,
                radius: true,
                dots: false,
            }),
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
