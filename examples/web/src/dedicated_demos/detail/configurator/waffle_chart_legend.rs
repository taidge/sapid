use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let position = scope.cage("bottom".to_owned());
        let options = scope.bond(move || PlotOptions {
            ..Default::default()
        });
        let palette = controls::chart_palette(&theme);
        let preview = plot("waffle-chart-legend", PlotKind::Waffle)
            .theme(theme.clone())
            .chart_theme(palette)
            .options(options)
            .frame(ChartFrame {
                width: 320.,
                height: 320.,
            })
            .data(controls::segment_example_data())
            .legend(true)
            .data_table(false)
            .navigation(false)
            .style(scope.bond(move || {
                Style::new()
                    .width(Length::Px(320.))
                    .max_width(Limit::Percent(100.))
            }));
        view()
            .class(position.map(|v| {
                format!(
                    "site-configurator site-chart-configurator site-compact-plot site-legend-{v}"
                )
            }))
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::segmented_choice(
                    "waffle-chart-legend-position",
                    "Legend position",
                    position,
                    &[
                        ("top", "top"),
                        ("bottom", "bottom"),
                        ("left", "left"),
                        ("right", "right"),
                    ],
                    &theme,
                )],
            ))
            .show_in(scope)
    })
}
