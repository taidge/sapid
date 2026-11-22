use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut palette = controls::chart_palette(&theme);
    palette.series[0] = Color(34, 139, 230, 255);
    let dial_0 = gauge_chart("gauge-round-caps-0")
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
            width: 160.,
            height: 122.,
        })
        .default_options(PlotOptions {
            angles: Some(PlotAngles {
                start: 150.,
                sweep: 240.,
            }),
            gauge_thickness: Some(12.),
            gauge_round_caps: true,
            gauge_value_label: true,
            tooltip: true,
            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(160.))
                .max_width(Limit::Percent(100.)),
        )
        .legend(false)
        .navigation(false)
        .data_table(false);
    let mut palette = controls::chart_palette(&theme);
    palette.series[0] = Color(34, 139, 230, 255);
    let dial_1 = gauge_chart("gauge-round-caps-1")
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
            width: 160.,
            height: 122.,
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
                .width(Length::Px(160.))
                .max_width(Limit::Percent(100.)),
        )
        .legend(false)
        .navigation(false)
        .data_table(false);
    let mut palette = controls::chart_palette(&theme);
    palette.series[..3].copy_from_slice(&[
        Color(18, 184, 134, 255),
        Color(250, 176, 5, 255),
        Color(250, 82, 82, 255),
    ]);
    let dial_2 = gauge_chart("gauge-round-caps-2")
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
            bands: vec![60., 80., 100.],
        }]))
        .frame(ChartFrame {
            width: 160.,
            height: 122.,
        })
        .default_options(PlotOptions {
            angles: Some(PlotAngles {
                start: 150.,
                sweep: 240.,
            }),
            gauge_thickness: Some(12.),
            gauge_round_caps: true,
            gauge_value_label: true,
            tooltip: true,
            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(160.))
                .max_width(Limit::Percent(100.)),
        )
        .legend(false)
        .navigation(false)
        .data_table(false);
    sapid::group()
        .theme(theme)
        .gap(16.)
        .fill((dial_0, dial_1, dial_2))
}
