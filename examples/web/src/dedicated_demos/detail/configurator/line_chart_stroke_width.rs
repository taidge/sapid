use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let width = scope.cage(2.);

        let data = controls::chart_example_data();

        let options = scope.bond(move || CartesianOptions {
            presentation: ChartPresentation {
                stroke_width: width.get(),
                ..Default::default()
            },
            ..Default::default()
        });
        let preview = chart("line-chart-stroke-width")
            .theme(theme.clone())
            .kind(CartesianKind::Line)
            .frame(ChartFrame {
                width: 520.,
                height: 300.,
            })
            .data(data)
            .options(options)
            .legend(false)
            .data_table(false)
            .navigation(false);
        view()
            .class("site-configurator site-chart-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::range(
                    "stroke-width",
                    "Stroke width",
                    width,
                    0.5,
                    5.,
                    0.1,
                    &theme,
                )],
            ))
            .show_in(scope)
    })
}
