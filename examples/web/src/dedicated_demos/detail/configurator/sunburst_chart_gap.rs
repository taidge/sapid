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
