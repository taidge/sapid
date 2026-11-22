use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut palette = controls::chart_palette(&theme);
    palette.series[0] = Color(34, 139, 230, 255);
    gauge_chart("gauge-start-and-end-angle-0")
        .theme(theme.clone())
        .name("Gauge")
        .chart_theme(palette)
        .data(PlotData::Targets(vec![TargetDatum {
            key: "cpu".into(),
            label: "CPU Usage".into(),
            min: 0.,
            max: 100.,
            value: 72.,
            target: None,
            bands: Vec::new(),
        }]))
        .frame(ChartFrame {
            width: 200.,
            height: 153.,
        })
        .default_options(PlotOptions {
            angles: Some(PlotAngles {
                start: 180.,
                sweep: 180.,
            }),
            gauge_thickness: Some(12.),
            gauge_round_caps: false,
            gauge_value_label: true,
            tooltip: true,
            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(200.))
                .max_width(Limit::Percent(100.)),
        )
        .legend(false)
        .navigation(false)
        .data_table(false)
}
