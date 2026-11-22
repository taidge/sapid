use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let plots = [false, true].map(|all| {
        let key = if all {
            "funnel-tooltip-all"
        } else {
            "funnel-tooltip-one"
        };
        let description = if all {
            "Data only for all segments"
        } else {
            "Data only for hovered segment"
        };
        let chart = funnel_chart(key)
            .theme(theme.clone())
            .name(description)
            .data(controls::segment_example_data())
            .chart_theme(controls::chart_palette(&theme))
            .frame(ChartFrame {
                width: 160.,
                height: 160.,
            })
            .default_options(PlotOptions {
                tooltip: true,
                tooltip_all_segments: all,
                ..Default::default()
            })
            .style(
                Style::new()
                    .width(Length::Px(160.))
                    .max_width(Limit::Percent(100.)),
            )
            .navigation(false)
            .legend(false)
            .data_table(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(8.)
            .fill((sapid::text().content(description), chart))
    });
    sapid::group()
        .theme(theme)
        .gap(50.)
        .fill(plots.into_iter().collect::<Vec<_>>())
}

#[allow(unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

    pub fn segment_example_data() -> sapid::charts::PlotData {
        use sapid::charts::*;
        PlotData::values(
            [
                ("usa", "USA", 400.),
                ("india", "India", 300.),
                ("japan", "Japan", 100.),
                ("other", "Other", 200.),
            ]
            .map(|(key, label, value)| ValueDatum::new(key, label, value)),
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
