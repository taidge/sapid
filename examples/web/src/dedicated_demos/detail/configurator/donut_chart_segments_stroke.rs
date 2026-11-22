use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let stroke = scope.cage(1.);
        let options = scope.bond(move || PlotOptions {
            angles: Some(PlotAngles {
                start: 0.,
                sweep: 360.,
            }),
            counterclockwise: true,
            tooltip: true,
            segment_stroke: Some(stroke.get()),
            ..Default::default()
        });
        let palette = controls::chart_palette(&theme);
        let preview = plot("donut-chart-segments-stroke", PlotKind::Donut)
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
                    .width(Length::Px(200.))
                    .max_width(Limit::Percent(100.))
            }));
        view()
            .class("site-configurator site-chart-configurator site-compact-plot")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::range(
                    "donut-chart-segments-stroke-stroke",
                    "Stroke width",
                    stroke,
                    0.,
                    5.,
                    0.1,
                    &theme,
                )],
            ))
            .show_in(scope)
    })
}
