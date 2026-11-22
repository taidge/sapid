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
        let preview = chart("line-chart-connect-nulls")
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
                vec![
                    controls::chart_curve_control("curve-type", curve, &theme),
                    controls::switch("Connect nulls", connect, &theme),
                ],
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

    pub fn select(
        key: &str,
        label: &str,
        value: Cage<Option<String>>,
        options: &'static [(&'static str, &'static str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let field_key = format!("line-chart-connect-nulls-{key}");
        let select_key = format!("line-chart-connect-nulls-select-{key}");
        Box::new(
            sapid::field(field_key)
                .theme(theme.clone())
                .label(label.to_owned())
                .control(move |field| {
                    let mut select = sapid::native_select(select_key.clone()).field(field);
                    for (value, label) in options {
                        select = select.option(sapid::select_option(*value, *label));
                    }
                    select.value(value).on_value_change(move |next| {
                        if next.is_some() {
                            value.revise(|current| *current = next)
                        }
                    })
                }),
        )
    }

    pub fn switch(label: &str, value: Cage<bool>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::switch()
                .theme(theme.clone())
                .size(Size::Small)
                .with_thumb_indicator(true)
                .label(label.to_owned())
                .checked(value)
                .on_checked_change(move |next| value.revise(|current| *current = next)),
        )
    }

    pub fn chart_curve(value: &str) -> sapid::charts::ChartCurve {
        use sapid::charts::ChartCurve::*;
        match value {
            "bump" => Bump,
            "natural" => Natural,
            "monotone" => Monotone,
            "step" => Step,
            "stepBefore" => StepBefore,
            "stepAfter" => StepAfter,
            _ => Linear,
        }
    }
    pub fn chart_curve_control(
        key: &str,
        value: Cage<Option<String>>,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        select(
            key,
            "Curve type",
            value,
            &[
                ("bump", "Bump"),
                ("linear", "Linear"),
                ("natural", "Natural"),
                ("monotone", "Monotone"),
                ("step", "Step"),
                ("stepBefore", "StepBefore"),
                ("stepAfter", "StepAfter"),
            ],
            theme,
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
