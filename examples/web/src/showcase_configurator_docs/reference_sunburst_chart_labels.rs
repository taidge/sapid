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
    let mut palette = controls::chart_palette(&theme);
    palette.series[..4].copy_from_slice(&[
        Color(230, 73, 128, 255),
        Color(190, 75, 219, 255),
        Color(18, 184, 134, 255),
        Color(34, 139, 230, 255),
    ]);
    palette.grid = theme.surface;
    sunburst_chart("reference-sunburst-chart-labels")
        .theme(theme)
        .name("Sunburst chart")
        .chart_theme(palette)
        .data(sample_data())
        .frame(ChartFrame {
            width: 320.,
            height: 320.,
        })
        .default_options(PlotOptions {
            counterclockwise: true,
            hierarchy_labels: true,
            sunburst_inner_ratio: Some(1. / 3.),
            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(400.))
                .max_width(Limit::Percent(100.)),
        )
        .legend(false)
        .navigation(false)
        .data_table(false)
}

#[allow(unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

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
