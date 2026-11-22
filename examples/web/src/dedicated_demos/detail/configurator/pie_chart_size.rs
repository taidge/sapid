use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let size = scope.cage(160.);
        let options = scope.bond(move || PlotOptions {
            angles: Some(PlotAngles {
                start: 0.,
                sweep: 360.,
            }),
            counterclockwise: true,
            ..Default::default()
        });
        let palette = controls::chart_palette(&theme);
        let preview = plot("pie-chart-size", PlotKind::Pie)
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
                    .width(Length::Px(size.get() * 320. / 280.))
                    .max_width(Limit::Percent(100.))
            }));
        view()
            .class("site-configurator site-chart-configurator site-compact-plot")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::range(
                    "pie-chart-size-size",
                    "Size",
                    size,
                    80.,
                    300.,
                    1.,
                    &theme,
                )],
            ))
            .show_in(scope)
    })
}
