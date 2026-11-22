use glory::prelude::*;
use sapid::charts::*;
fn sample_data() -> PlotData {
    PlotData::Flows(FlowData {
        nodes: [
            ChartCategory::new("visit", "Visit"),
            ChartCategory::new("direct", "Direct-Favourite"),
            ChartCategory::new("page", "Page-Click"),
            ChartCategory::new("detail", "Detail-Favourite"),
            ChartCategory::new("lost", "Lost"),
        ]
        .into(),
        links: [
            ("visit", "direct", 3728.3),
            ("visit", "page", 354170.0),
            ("page", "detail", 62429.0),
            ("page", "lost", 291741.0),
        ]
        .into_iter()
        .map(|(from, to, value)| FlowLink {
            key: format!("{from}-{to}"),
            source: from.into(),
            target: to.into(),
            value,
        })
        .collect(),
    })
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut palette = controls::chart_palette(&theme);
    palette.series[..5].copy_from_slice(&[
        Color(34, 139, 230, 255),
        Color(134, 142, 150, 255),
        Color(18, 184, 134, 255),
        Color(81, 207, 102, 255),
        Color(130, 201, 30, 255),
    ]);
    sankey_chart("sankey-chart-overview")
        .theme(theme)
        .name("Sankey chart")
        .chart_theme(palette)
        .data(sample_data())
        .frame(ChartFrame {
            width: 684.,
            height: 300.,
        })
        .default_options(PlotOptions {
            link_opacity: Some(0.035),
            ..Default::default()
        })
        .style(Style::new().width(Length::Percent(100.)))
        .legend(false)
        .navigation(false)
        .data_table(false)
}
