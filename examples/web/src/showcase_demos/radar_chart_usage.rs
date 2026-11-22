use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = controls::chart_palette(&theme);
    radar_chart("radar-chart-overview")
        .theme(theme)
        .name("Product sales")
        .chart_theme(palette)
        .data(PlotData::values([
            ValueDatum::new("group0", "Apples", 120.0),
            ValueDatum::new("group1", "Oranges", 98.0),
            ValueDatum::new("group2", "Tomatoes", 86.0),
            ValueDatum::new("group3", "Grapes", 99.0),
            ValueDatum::new("group4", "Bananas", 85.0),
            ValueDatum::new("group5", "Lemons", 65.0),
        ]))
        .frame(ChartFrame {
            width: 320.,
            height: 320.,
        })
        .default_options(PlotOptions {
            radar_parts: Some(RadarParts {
                grid: true,
                angles: true,
                radius: true,
                dots: false,
            }),
            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(300.))
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
