use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = {
        let mut p = controls::chart_palette(&theme);
        p.series.fill(glory::style::Color(18, 184, 134, 255));
        p
    };
    bars_list("reference-bars-list-bar-color")
        .theme(theme)
        .name("Downloads by framework")
        .chart_theme(palette)
        .data(PlotData::values([
            ValueDatum::new("group0", "React", 950000.0),
            ValueDatum::new("group1", "Vue", 320000.0),
            ValueDatum::new("group2", "Angular", 580000.0),
            ValueDatum::new("group3", "Svelte", 145000.0),
            ValueDatum::new("group4", "Next.js", 720000.0),
            ValueDatum::new("group5", "Nuxt", 180000.0),
            ValueDatum::new("group6", "Remix", 95000.0),
        ]))
        .frame(ChartFrame {
            width: 640.,
            height: 320.,
        })
        .default_options(PlotOptions {
            bar_labels: true,
            ..Default::default()
        })
        .style(Style::new().width(Length::Percent(100.)))
        .legend(false)
        .navigation(false)
        .data_table(false)
}
