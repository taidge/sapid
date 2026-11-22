use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let gap = scope.cage(2.);
        let columns = scope.cage(10.);
        let rows = scope.cage(10.);
        let options = scope.bond(move || PlotOptions {
            waffle_grid: Some(WaffleGrid {
                rows: rows.get() as u8,
                columns: columns.get() as u8,
                gap: gap.get(),
            }),
            ..Default::default()
        });
        let palette = controls::chart_palette(&theme);
        let preview = plot("waffle-chart-gap-and-grid-size", PlotKind::Waffle)
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
                vec![
                    controls::range(
                        "waffle-chart-gap-and-grid-size-gap",
                        "Gap",
                        gap,
                        0.,
                        5.,
                        1.,
                        &theme,
                    ),
                    controls::range(
                        "waffle-chart-gap-and-grid-size-columns",
                        "Columns",
                        columns,
                        5.,
                        15.,
                        1.,
                        &theme,
                    ),
                    controls::range(
                        "waffle-chart-gap-and-grid-size-rows",
                        "Rows",
                        rows,
                        5.,
                        15.,
                        1.,
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
