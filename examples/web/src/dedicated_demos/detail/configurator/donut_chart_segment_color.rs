use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage("Blue".to_owned());
        let base = theme.clone();
        let options = scope.bond(move || PlotOptions {
            angles: Some(PlotAngles {
                start: 0.,
                sweep: 360.,
            }),
            counterclockwise: true,
            tooltip: true,
            ..Default::default()
        });
        let palette = scope.bond(move || {
            let mut palette = controls::chart_palette(&base);
            palette.series[0] = controls::accent(base.clone(), &color.get()).accent;
            palette
        });
        let preview = plot("donut-chart-segment-color", PlotKind::Donut)
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
                vec![controls::named_color_choices(
                    "donut-chart-segment-color-color",
                    "Color",
                    color.into(),
                    move |next| color.revise(|current| *current = next),
                    &theme,
                )],
            ))
            .show_in(scope)
    })
}
