use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = {
        let mut p = controls::chart_palette(&theme);
        p.series[..4].copy_from_slice(&[
            glory::style::Color(34, 139, 230, 255),
            glory::style::Color(18, 184, 134, 255),
            glory::style::Color(253, 126, 20, 255),
            glory::style::Color(134, 142, 150, 255),
        ]);
        p
    };
    waffle_chart("reference-waffle-chart-tooltip")
        .theme(theme)
        .name("Browser market share")
        .chart_theme(palette)
        .data(PlotData::values([
            ValueDatum::new("group0", "Chrome", 65.0),
            ValueDatum::new("group1", "Safari", 19.0),
            ValueDatum::new("group2", "Firefox", 10.0),
            ValueDatum::new("group3", "Other", 6.0),
        ]))
        .frame(ChartFrame {
            width: 320.,
            height: 320.,
        })
        .default_options(PlotOptions {
            tooltip: true,
            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(252.))
                .max_width(Limit::Percent(100.)),
        )
        .legend(true)
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
