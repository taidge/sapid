use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let categories = ["Mar 22", "Mar 23", "Mar 24", "Mar 25", "Mar 26"]
        .into_iter()
        .enumerate()
        .map(|(i, label)| ChartCategory::new(format!("c{i}"), label))
        .collect();
    controls::cartesian_playground(
        theme,
        "area-chart-overview",
        CartesianKind::Area,
        ChartData {
            categories,
            series: vec![
                ChartSeries::new(
                    "apples",
                    "Apples",
                    [2890.0, 2756.0, 3322.0, 3470.0, 3129.0].map(Some),
                ),
                ChartSeries::new(
                    "oranges",
                    "Oranges",
                    [2338.0, 2103.0, 986.0, 2108.0, 1726.0].map(Some),
                ),
                ChartSeries::new(
                    "tomatoes",
                    "Tomatoes",
                    [2452.0, 2402.0, 1821.0, 2809.0, 2290.0].map(Some),
                ),
            ],
        },
    )
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
        let field_key = format!("control-{key}");
        let select_key = format!("control-select-{key}");
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

    pub fn segmented_choice(
        key: &'static str,
        label: &'static str,
        value: Cage<String>,
        options: &'static [(&str, &str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        Box::new(
            sapid::segmented_control(key)
                .theme(theme.clone())
                .size(Size::Small)
                .label(label)
                .full_width(true)
                .value(value)
                .segments(
                    options
                        .iter()
                        .map(|&(key, caption)| sapid::segment(key).label(caption)),
                )
                .on_value_change(move |next| value.revise(|current| *current = next)),
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
    pub fn chart_axes(value: &str) -> sapid::charts::ChartAxes {
        use sapid::charts::ChartAxes::*;
        match value {
            "x" => X,
            "y" => Y,
            "xy" => Both,
            _ => None,
        }
    }
    pub fn cartesian_playground(
        theme: Theme,
        key: &'static str,
        kind: sapid::charts::CartesianKind,
        data: sapid::charts::ChartData,
    ) -> impl Widget {
        use sapid::charts::*;
        widget(move |scope| {
            let curve = scope.cage(Some("linear".to_owned()));
            let grid = scope.cage("x".to_owned());
            let ticks = scope.cage("y".to_owned());
            let x_axis = scope.cage(true);
            let y_axis = scope.cage(true);
            let dots = scope.cage(kind != CartesianKind::Bar);
            let gradient = scope.cage(kind == CartesianKind::Area);
            let options = scope.bond(move || CartesianOptions {
                domain: if matches!(kind, CartesianKind::Line | CartesianKind::Area) {
                    Some(ChartDomain::new(0., 3600.).unwrap())
                } else {
                    None
                },
                presentation: ChartPresentation {
                    edge_categories: true,
                    curve: chart_curve(curve.get().as_deref().unwrap_or("linear")),
                    grid: chart_axes(&grid.get()),
                    tick_lines: chart_axes(&ticks.get()),
                    x_axis: x_axis.get(),
                    y_axis: y_axis.get(),
                    dots: dots.get(),
                    area_gradient: gradient.get(),
                    fill_opacity: 0.2,
                    ..Default::default()
                },
                ..Default::default()
            });
            let mut palette = ChartTheme::from_theme(&theme);
            palette.series[..3].copy_from_slice(&[
                Color(76, 110, 245, 255),
                Color(34, 139, 230, 255),
                Color(18, 184, 134, 255),
            ]);
            let preview = chart(key)
                .kind(kind)
                .theme(theme.clone())
                .name("Monthly series")
                .frame(ChartFrame {
                    width: 520.,
                    height: 300.,
                })
                .chart_theme(palette)
                .options(options)
                .data(data.clone())
                .data_table(false)
                .legend(false)
                .navigation(false);
            let axes = &[("x", "x"), ("y", "y"), ("xy", "xy"), ("none", "none")];
            let mut settings = Vec::new();
            if kind != CartesianKind::Bar {
                settings.push(chart_curve_control("curve-type", curve, &theme));
            }
            settings.push(segmented_choice(
                "primary-ticks",
                "Tick line",
                ticks,
                axes,
                &theme,
            ));
            settings.push(segmented_choice(
                "primary-grid",
                "Grid axis",
                grid,
                axes,
                &theme,
            ));
            if kind == CartesianKind::Area {
                settings.push(switch("With gradient", gradient, &theme));
            }
            settings.push(switch("With xaxis", x_axis, &theme));
            settings.push(switch("With yaxis", y_axis, &theme));
            if kind != CartesianKind::Bar {
                settings.push(switch("With dots", dots, &theme));
            }
            view()
                .class("site-configurator site-chart-configurator")
                .fill(playground(Box::new(preview), settings))
                .show_in(scope)
        })
    }
}
