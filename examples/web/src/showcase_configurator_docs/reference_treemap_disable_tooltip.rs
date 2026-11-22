use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = controls::chart_palette(&theme);
    treemap("reference-treemap-disable-tooltip")
        .theme(theme)
        .name("Treemap chart")
        .chart_theme(palette)
        .data(PlotData::Hierarchy(
            [
                ("usa", "USA", 400.),
                ("india", "India", 300.),
                ("japan", "Japan", 100.),
                ("other", "Other", 200.),
            ]
            .into_iter()
            .map(|(k, label, v)| HierarchyDatum {
                key: k.into(),
                label: label.into(),
                weight: Some(v),
                parent: None,
            })
            .collect(),
        ))
        .frame(ChartFrame {
            width: 684.,
            height: 300.,
        })
        .default_options(PlotOptions {
            tooltip: false,
            ..Default::default()
        })
        .style(Style::new().width(Length::Percent(100.)))
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
