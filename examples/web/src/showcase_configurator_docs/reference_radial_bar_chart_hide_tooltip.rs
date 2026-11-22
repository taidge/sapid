use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = controls::chart_palette(&theme);
    radial_bar_chart("reference-radial-bar-chart-hide-tooltip")
        .theme(theme)
        .name("Distribution by age")
        .chart_theme(palette)
        .data(PlotData::values([
            ValueDatum::new("group0", "18-24", 31.47),
            ValueDatum::new("group1", "25-29", 26.69),
            ValueDatum::new("group2", "30-34", 15.69),
            ValueDatum::new("group3", "35-39", 8.22),
            ValueDatum::new("group4", "40-49", 8.63),
            ValueDatum::new("group5", "50+", 2.63),
            ValueDatum::new("group6", "unknown", 6.67),
        ]))
        .frame(ChartFrame {
            width: 320.,
            height: 320.,
        })
        .default_options(PlotOptions {
            tooltip: false,
            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(260.))
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
