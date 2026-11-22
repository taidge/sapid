use glory::prelude::*;
use sapid::charts::*;
fn revenue(
    theme: &sapid::Theme,
    key: &str,
    thickness: f64,
    vertical: bool,
    target: Option<f64>,
    custom_color: bool,
    tooltip: bool,
) -> Plot {
    let mut palette = ChartTheme::from_theme(theme);
    palette.series[..3].copy_from_slice(&[
        Color(224, 49, 49, 255),
        Color(240, 140, 0, 255),
        Color(9, 146, 104, 255),
    ]);
    let (width, height, display_width) = if vertical {
        (160., 500., 80.)
    } else {
        (800., (thickness + 52.0_f64).max(120.), 400.)
    };
    bullet_chart(key)
        .theme(theme.clone())
        .name("Revenue")
        .chart_theme(palette)
        .frame(ChartFrame { width, height })
        .data(PlotData::Targets(vec![TargetDatum {
            key: "revenue".into(),
            label: "Revenue".into(),
            min: 0.,
            max: 300000.,
            value: 230000.,
            target,
            bands: vec![100000., 200000., 300000.],
        }]))
        .default_options(PlotOptions {
            bullet_track: Some(BulletTrack {
                vertical,
                thickness,
                bar: if custom_color {
                    Color(0, 0, 0, 255)
                } else {
                    Color(255, 255, 255, 255)
                },
                target: if custom_color {
                    Color(255, 255, 255, 255)
                } else {
                    Color(33, 37, 41, 255)
                },
                label_divisor: 1000.,
                label_prefix: "$".into(),
                label_suffix: "k".into(),
            }),
            tooltip,
            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(display_width))
                .max_width(Limit::Percent(100.)),
        )
        .legend(false)
        .navigation(false)
        .data_table(false)
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill(
        [
            ("small", "Small", 40.),
            ("default", "Default", 64.),
            ("large", "Large", 96.),
        ]
        .map(|(key, label, thickness)| {
            sapid::stack().theme(theme.clone()).gap(2.).fill((
                sapid::text().content(label),
                revenue(&theme, key, thickness, false, Some(150000.), false, false),
            ))
        })
        .into_iter()
        .collect::<Vec<_>>(),
    )
}
