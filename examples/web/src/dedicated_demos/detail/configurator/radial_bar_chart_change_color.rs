use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage("Blue".to_owned());
        let base = theme.clone();

        let palette = scope.bond(move || {
            let mut palette = ChartTheme::from_theme(&base);
            palette.grid = base.disabled;
            palette
                .series
                .fill(controls::accent(base.clone(), &color.get()).accent);
            palette
        });
        let preview = plot("radial-bar-chart-change-color", PlotKind::RadialBar)
            .theme(theme.clone())
            .chart_theme(palette)
            .frame(ChartFrame {
                width: 320.,
                height: 320.,
            })
            .data(PlotData::values(
                [
                    ("unknown", "Unknown", 6.67),
                    ("age-50-up", "50+", 2.63),
                    ("age-40", "40-49", 8.63),
                    ("age-35", "35-39", 8.22),
                    ("age-30", "30-34", 15.69),
                    ("age-25", "25-29", 26.69),
                    ("age-18", "18-24", 31.47),
                ]
                .map(|(key, age, count)| ValueDatum::new(key, age, count)),
            ))
            .legend(false)
            .data_table(false)
            .navigation(false)
            .style(scope.bond(move || {
                Style::new()
                    .width(Length::Px(220.))
                    .max_width(Limit::Percent(100.))
            }));
        view()
            .class("site-configurator site-chart-configurator site-compact-plot site-radial-color-demo")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::named_color_choices(
                    "radial-bar-chart-change-color-color",
                    "Color",
                    color.into(),
                    move |next| color.revise(|current| *current = next),
                    &theme,
                )],
            ))
            .show_in(scope)
    })
}
