use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = controls::chart_palette(&theme);
    radial_bar_chart("reference-radial-bar-chart-legend")
        .theme(theme)
        .name("Distribution by age")
        .chart_theme(palette)
        .data(PlotData::values([
            ValueDatum::new("group0", "18-24", 31.47),
            ValueDatum::new("group1", "25-29", 26.69),
            ValueDatum::new("group2", "30-34", 15.69),
            ValueDatum::new("group3", "35-39", 8.22),
            ValueDatum::new("group4", "40-49", 8.63),
            ValueDatum::new("group5", "50+", 2.63),
            ValueDatum::new("group6", "unknown", 6.67),
        ]))
        .frame(ChartFrame {
            width: 320.,
            height: 320.,
        })
        .default_options(PlotOptions {
            tooltip: true,
            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(260.))
                .max_width(Limit::Percent(100.)),
        )
        .legend(true)
        .navigation(false)
        .data_table(false)
}
