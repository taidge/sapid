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
        let preview = chart("composite-chart-stroke-width")
            .theme(theme.clone())
            .kind(CartesianKind::Composite)
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

#[allow(unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

    pub fn playground(preview: Box<dyn Widget>, controls: Vec<Box<dyn Widget>>) -> Box<dyn Widget> {
        Box::new(
            view()
                .class("site-button-playground")
                .style(
                    Style::new()
                        .display(Display::Flex)
                        .flex_wrap(FlexWrap::Wrap)
                        .gap(24.0),
                )
                .fill((
                    col()
                        .class("site-playground-preview")
                        .style(Style {
                            grow: Some(1.0),
                            align: Some(Align::Center),
                            justify: Some(Justify::Center),
                            ..Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .min_width(Length::Px(180.0))
                        })
                        .fill(preview),
                    col()
                        .class("site-playground-controls")
                        .style(
                            Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .width(Length::Px(240.0))
                                .max_width(Limit::Percent(100.0)),
                        )
                        .fill(controls),
                )),
        )
    }

    pub fn range(
        key: &str,
        label: &str,
        value: Cage<f64>,
        min: f64,
        max: f64,
        step: f64,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let label = label.to_owned();
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content(value.map({
                        let label = label.clone();
                        move |v| {
                            let decimals = if step.fract() == 0. {
                                0
                            } else if (step * 10.).fract() == 0. {
                                1
                            } else {
                                2
                            };
                            format!("{label} · {v:.decimals$}")
                        }
                    })),
                sapid::slider(key.to_owned())
                    .theme(theme.clone())
                    .name(label)
                    .min(min)
                    .max(max)
                    .step(step)
                    .value(value)
                    .on_value_change(move |next| value.revise(|v| *v = next)),
            )),
        )
    }
    pub fn chart_example_data() -> sapid::charts::ChartData {
        use sapid::charts::*;
        ChartData {
            categories: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"]
                .into_iter()
                .map(|month| ChartCategory::new(month, month))
                .collect(),
            series: vec![
                ChartSeries::new("apples", "Apples", [12., 18., 14., 20., 17., 24.].map(Some)),
                ChartSeries::new("pears", "Pears", [8., 10., 9., 12., 11., 13.].map(Some))
                    .kind(ChartSeriesKind::Bar),
            ],
        }
    }
}
