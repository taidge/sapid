use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let leader = scope.cage(true);
        let position = scope.cage("outside".to_owned());
        let value = scope.cage("value".to_owned());
        let options = scope.bond(move || PlotOptions {
            angles: Some(PlotAngles {
                start: 0.,
                sweep: 360.,
            }),
            counterclockwise: true,
            segment_labels: Some(SegmentLabels {
                position: if position.get() == "inside" {
                    SegmentLabelPosition::Inside
                } else {
                    SegmentLabelPosition::Outside
                },
                value: match value.get().as_str() {
                    "percent" => SegmentLabelValue::Percent,
                    "name" => SegmentLabelValue::Name,
                    _ => SegmentLabelValue::Value,
                },
                leader: leader.get(),
            }),
            ..Default::default()
        });
        let palette = controls::chart_palette(&theme);
        let preview = plot("pie-chart-segments-labels", PlotKind::Pie)
            .theme(theme.clone())
            .chart_theme(palette)
            .options(options)
            .frame(ChartFrame {
                width: 320.,
                height: 320.,
            })
            .data(controls::segment_example_data())
            .legend(false)
            .data_table(false)
            .navigation(false)
            .style(scope.bond(move || {
                Style::new()
                    .width(Length::Px(300.))
                    .max_width(Limit::Percent(100.))
            }));
        view()
            .class("site-configurator site-chart-configurator site-compact-plot")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("With labels line", leader, &theme),
                    controls::segmented_choice(
                        "pie-chart-segments-labels-position",
                        "Labels position",
                        position,
                        &[("inside", "inside"), ("outside", "outside")],
                        &theme,
                    ),
                    controls::segmented_choice(
                        "pie-chart-segments-labels-value",
                        "Labels type",
                        value,
                        &[("value", "value"), ("percent", "percent"), ("name", "name")],
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
