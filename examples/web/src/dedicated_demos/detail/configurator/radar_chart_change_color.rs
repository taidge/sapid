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
        ("speed", "Speed", 80.0),
        ("quality", "Quality", 65.0),
        ("reliability", "Reliability", 90.0),
        ("support", "Support", 72.0),
    ])
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage("Blue".to_owned());
        let base = theme.clone();
        let stroke = scope.cage("Blue".to_owned());
        let stroke_theme = theme.clone();
        let options = scope.bond(move || PlotOptions {
            radar_stroke: Some(controls::accent(stroke_theme.clone(), &stroke.get()).accent),
            ..Default::default()
        });
        let palette = scope.bond(move || {
            let mut palette = controls::chart_palette(&base);
            palette.series[0] = controls::accent(base.clone(), &color.get()).accent;
            palette
        });
        let preview = plot("radar-chart-change-color", PlotKind::Radar)
            .theme(theme.clone())
            .chart_theme(palette)
            .options(options)
            .frame(ChartFrame {
                width: 320.,
                height: 320.,
            })
            .data(sample_data())
            .legend(false)
            .data_table(false)
            .navigation(false)
            .style(scope.bond(move || {
                Style::new()
                    .width(Length::Px(200.))
                    .max_width(Limit::Percent(100.))
            }));
        view()
            .class("site-configurator site-chart-configurator site-compact-plot")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::named_color_choices(
                        "radar-chart-change-color-color",
                        "Color",
                        color.into(),
                        move |next| color.revise(|current| *current = next),
                        &theme,
                    ),
                    controls::named_color_choices(
                        "radar-chart-change-color-stroke",
                        "Stroke color",
                        stroke.into(),
                        move |next| stroke.revise(|current| *current = next),
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
