use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage("Blue".to_owned());
        let base = theme.clone();
        let palette = scope.bond(move || {
            let mut palette = ChartTheme::from_theme(&base);
            palette.series[0] = controls::accent(base.clone(), &color.get()).accent;
            palette
        });
        let data = controls::chart_example_data();

        let options = scope.bond(move || CartesianOptions {
            presentation: ChartPresentation {
                ..Default::default()
            },
            ..Default::default()
        });
        let preview = chart("line-chart-line-color")
            .theme(theme.clone())
            .kind(CartesianKind::Line)
            .frame(ChartFrame {
                width: 520.,
                height: 300.,
            })
            .data(data)
            .chart_theme(palette)
            .options(options)
            .legend(false)
            .data_table(false)
            .navigation(false);
        view()
            .class("site-configurator site-chart-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::color_choices(color, &theme)],
            ))
            .show_in(scope)
    })
}
