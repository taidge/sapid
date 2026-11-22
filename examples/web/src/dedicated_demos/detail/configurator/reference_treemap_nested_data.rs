use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = controls::chart_palette(&theme);
    treemap("reference-treemap-nested-data")
        .theme(theme)
        .name("Treemap chart")
        .chart_theme(palette)
        .data(PlotData::Hierarchy(
            [
                ("frontend", "Frontend", None, None),
                ("react", "React", Some("frontend"), Some(400.)),
                ("vue", "Vue", Some("frontend"), Some(200.)),
                ("angular", "Angular", Some("frontend"), Some(150.)),
                ("backend", "Backend", None, None),
                ("node", "Node", Some("backend"), Some(300.)),
                ("python", "Python", Some("backend"), Some(250.)),
                ("go", "Go", Some("backend"), Some(100.)),
                ("mobile", "Mobile", None, None),
                ("react-native", "React Native", Some("mobile"), Some(200.)),
                ("flutter", "Flutter", Some("mobile"), Some(180.)),
            ]
            .into_iter()
            .map(|(key, label, parent, weight)| HierarchyDatum {
                key: key.into(),
                label: label.into(),
                parent: parent.map(str::to_owned),
                weight,
            })
            .collect(),
        ))
        .frame(ChartFrame {
            width: 684.,
            height: 300.,
        })
        .default_options(PlotOptions {
            tooltip: true,
            ..Default::default()
        })
        .style(Style::new().width(Length::Percent(100.)))
        .legend(false)
        .navigation(false)
        .data_table(false)
}
