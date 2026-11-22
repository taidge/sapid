use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let position = scope.cage("right".to_owned());
        let options = scope.bond(move || PlotOptions {
            segment_labels: Some(SegmentLabels {
                position: match position.get().as_str() {
                    "left" => SegmentLabelPosition::Left,
                    "inside" => SegmentLabelPosition::Inside,
                    _ => SegmentLabelPosition::Right,
                },
                value: SegmentLabelValue::Name,
                leader: false,
            }),
            ..Default::default()
        });
        let palette = controls::chart_palette(&theme);
        let preview = plot("funnel-chart-segments-labels", PlotKind::Funnel)
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
                vec![controls::segmented_choice(
                    "funnel-chart-segments-labels-position",
                    "Labels position",
                    position,
                    &[("left", "left"), ("inside", "inside"), ("right", "right")],
                    &theme,
                )],
            ))
            .show_in(scope)
    })
}
