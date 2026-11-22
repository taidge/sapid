use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = controls::chart_palette(&theme);
    funnel_chart("reference-funnel-chart-without-tooltip")
        .theme(theme)
        .name("Value by country")
        .chart_theme(palette)
        .data(controls::segment_example_data())
        .frame(ChartFrame {
            width: 340.,
            height: 300.,
        })
        .default_options(PlotOptions {
            tooltip: false,
            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(340.))
                .max_width(Limit::Percent(100.)),
        )
        .legend(false)
        .navigation(false)
        .data_table(false)
}
