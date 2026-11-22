use glory::prelude::*;
use sapid::charts::*;
fn sample_data() -> PlotData {
    PlotData::Hierarchy(
        [
            ("analytics", "Analytics", None, Some(100.)),
            ("devops", "DevOps", None, None),
            ("docker", "Docker", Some("devops"), Some(80.)),
            ("kubernetes", "Kubernetes", Some("devops"), Some(50.)),
            ("backend", "Backend", None, None),
            ("node", "Node", Some("backend"), Some(150.)),
            ("python", "Python", Some("backend"), None),
            ("django", "Django", Some("python"), Some(110.)),
            ("fastapi", "FastAPI", Some("python"), Some(60.)),
            ("go", "Go", Some("backend"), Some(50.)),
            ("frontend", "Frontend", None, None),
            ("react", "React", Some("frontend"), None),
            ("frameworks", "Frameworks", Some("react"), None),
            ("next-js", "Next.js", Some("frameworks"), Some(150.)),
            ("remix", "Remix", Some("frameworks"), Some(40.)),
            ("cra", "CRA", Some("react"), Some(20.)),
            ("vue", "Vue", Some("frontend"), Some(90.)),
            ("svelte", "Svelte", Some("frontend"), Some(30.)),
        ]
        .into_iter()
        .map(|(key, label, parent, weight)| HierarchyDatum {
            key: key.into(),
            label: label.into(),
            parent: parent.map(str::to_owned),
            weight,
        })
        .collect(),
    )
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let gap = scope.cage(1.);
        let options = scope.bond(move || PlotOptions {
            counterclockwise: true,
            sunburst_inner_ratio: Some(1. / 3.),
            hierarchy_gap: Some(gap.get()),
            ..Default::default()
        });
        let mut palette = controls::chart_palette(&theme);
        palette.series[..4].copy_from_slice(&[
            Color(230, 73, 128, 255),
            Color(190, 75, 219, 255),
            Color(18, 184, 134, 255),
            Color(34, 139, 230, 255),
        ]);
        palette.grid = theme.surface;
        let preview = plot("sunburst-chart-gap", PlotKind::Sunburst)
            .theme(theme.clone())
            .chart_theme(palette)
            .options(options)
            .frame(ChartFrame {
                width: 320.,
                height: 320.,
            })
            .data(sample_data())
            .legend(false)
            .data_table(false)
            .navigation(false)
            .style(scope.bond(move || {
                Style::new()
                    .width(Length::Px(300.))
                    .max_width(Limit::Percent(100.))
            }));
        view()
            .class("site-configurator site-chart-configurator site-compact-plot")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::range(
                    "sunburst-chart-gap-gap",
                    "Gap",
                    gap,
                    0.,
                    10.,
                    1.,
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
    pub fn chart_palette(theme: &Theme) -> sapid::charts::ChartTheme {
        let mut palette = sapid::charts::ChartTheme::from_theme(theme);
        palette.series[..4].copy_from_slice(&[
            Color(76, 110, 245, 255),
            Color(250, 176, 5, 255),
            Color(18, 184, 134, 255),
            Color(134, 142, 150, 255),
        ]);
        palette
    }
}
