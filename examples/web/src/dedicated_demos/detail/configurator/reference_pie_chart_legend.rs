use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = controls::chart_palette(&theme);
    pie_chart("reference-pie-chart-legend")
        .theme(theme)
        .name("Share by country")
        .chart_theme(palette)
        .data(controls::segment_example_data())
        .frame(ChartFrame {
            width: 320.,
            height: 320.,
        })
        .default_options(PlotOptions {
            angles: Some(PlotAngles {
                start: 0.,
                sweep: 360.,
            }),
            counterclockwise: true,
            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(184.))
                .max_width(Limit::Percent(100.)),
        )
        .legend(true)
        .navigation(false)
        .data_table(false)
}
