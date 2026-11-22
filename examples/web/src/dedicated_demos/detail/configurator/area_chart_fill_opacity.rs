use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let chart_type = scope.cage("default".to_owned());
        let opacity = scope.cage(0.2);
        let gradient = scope.cage(true);

        let data = controls::chart_example_data();

        let options = scope.bond(move || CartesianOptions {
            stacked: chart_type.get() == "stacked",
            presentation: ChartPresentation {
                fill_opacity: opacity.get(),
                area_gradient: gradient.get(),
                ..Default::default()
            },
            ..Default::default()
        });
        let preview = chart("area-chart-fill-opacity")
            .theme(theme.clone())
            .kind(CartesianKind::Area)
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
                vec![
                    controls::segmented_choice(
                        "area-type",
                        "Type",
                        chart_type,
                        &[("default", "default"), ("stacked", "stacked")],
                        &theme,
                    ),
                    controls::range(
                        "fill-opacity",
                        "Fill opacity",
                        opacity,
                        0.,
                        1.,
                        0.01,
                        &theme,
                    ),
                    controls::switch("With gradient", gradient, &theme),
                ],
            ))
            .show_in(scope)
    })
}
