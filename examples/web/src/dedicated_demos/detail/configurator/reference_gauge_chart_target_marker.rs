use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut palette = controls::chart_palette(&theme);
    palette.series[..3].copy_from_slice(&[
        Color(18, 184, 134, 255),
        Color(250, 176, 5, 255),
        Color(250, 82, 82, 255),
    ]);
    gauge_chart("gauge-target-marker-0")
        .theme(theme.clone())
        .name("Gauge")
        .chart_theme(palette)
        .data(PlotData::Targets(vec![TargetDatum {
            key: "cpu".into(),
            label: "CPU Usage".into(),
            min: 0.,
            max: 100.,
            value: 72.,
            target: Some(75.),
            bands: vec![60., 80., 100.],
        }]))
        .frame(ChartFrame {
            width: 200.,
            height: 153.,
        })
        .default_options(PlotOptions {
            angles: Some(PlotAngles {
                start: 150.,
                sweep: 240.,
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
