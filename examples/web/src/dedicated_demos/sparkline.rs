use glory::prelude::*;
use sapid::charts::*;
fn values(values: &[(&str, &str, f64)]) -> PlotData {
    PlotData::Values(
        values
            .iter()
            .map(|(key, label, value)| ValueDatum::new(*key, *label, *value))
            .collect(),
    )
}
fn sample_data() -> PlotData {
    values(&[
        ("w1", "Week 1", 4.0),
        ("w2", "Week 2", 7.0),
        ("w3", "Week 3", 5.0),
        ("w4", "Week 4", 9.0),
        ("w5", "Week 5", 8.0),
        ("w6", "Week 6", 12.0),
        ("w7", "Week 7", 15.0),
    ])
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let curve = scope.cage(Some("linear".to_owned()));
        let color = scope.cage("Blue".to_owned());
        let opacity = scope.cage(0.6);
        let gradient = scope.cage(true);
        let width = scope.cage(2.);
        let base = theme.clone();
        let palette = scope.bond(move || {
            let mut p = controls::chart_palette(&base);
            p.series[0] = controls::accent(base.clone(), &color.get()).accent;
            p
        });
        let options = scope.bond(move || PlotOptions {
            sparkline: Some(SparklineStyle {
                curve: controls::chart_curve(curve.get().as_deref().unwrap_or("linear")),
                stroke_width: width.get(),
                fill_opacity: opacity.get(),
                gradient: gradient.get(),
            }),
            ..Default::default()
        });
        let preview = sparkline("sparkline-overview")
            .name("Weekly values: 4, 7, 5, 9, 8, 12, 15")
            .theme(theme.clone())
            .data(sample_data())
            .chart_theme(palette)
            .options(options)
            .legend(false)
            .navigation(false)
            .data_table(false)
            .frame(ChartFrame {
                width: 640.,
                height: 160.,
            })
            .style(
                Style::new()
                    .width(Length::Px(200.))
                    .max_width(Limit::Percent(100.)),
            );
        let settings = vec![
            controls::chart_curve_control("spark-curve", curve, &theme),
            controls::color_choices(color, &theme),
            controls::range(
                "spark-opacity",
                "Fill opacity",
                opacity,
                0.,
                1.,
                0.01,
                &theme,
            ),
            controls::switch("With gradient", gradient, &theme),
            controls::range("spark-width", "Stroke width", width, 0.5, 5., 0.1, &theme),
        ];
        view()
            .class("site-configurator site-chart-configurator site-compact-plot")
            .fill(controls::playground(Box::new(preview), settings))
            .show_in(scope)
    })
}
