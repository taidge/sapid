use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let curve = scope.cage(Some("linear".to_owned()));
        let connect = scope.cage(true);

        let mut data = controls::chart_example_data();
        data.series[0].values[2] = None;
        let options = scope.bond(move || CartesianOptions {
            presentation: ChartPresentation {
                curve: controls::chart_curve(curve.get().as_deref().unwrap_or("linear")),
                connect_nulls: connect.get(),
                ..Default::default()
            },
            ..Default::default()
        });
        let preview = chart("area-chart-connect-nulls")
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
                    controls::chart_curve_control("curve-type", curve, &theme),
                    controls::switch("Connect nulls", connect, &theme),
                ],
            ))
            .show_in(scope)
    })
}
