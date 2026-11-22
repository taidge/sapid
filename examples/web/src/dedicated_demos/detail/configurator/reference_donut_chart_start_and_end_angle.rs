use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = controls::chart_palette(&theme);
    donut_chart("reference-donut-chart-start-and-end-angle")
        .theme(theme)
        .name("Share by country")
        .chart_theme(palette)
        .data(controls::segment_example_data())
        .frame(ChartFrame {
            width: 320.,
            height: 320.,
        })
        .default_options(PlotOptions {
            tooltip: true,
            angles: Some(PlotAngles {
                start: 180.,
                sweep: 180.,
            }),
            donut_inner_ratio: Some(0.75),
            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(184.))
                .max_width(Limit::Percent(100.)),
        )
        .legend(false)
        .navigation(false)
        .data_table(false)
}
