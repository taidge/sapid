use glory::prelude::*;
use sapid::charts::*;
fn points() -> PlotData {
    PlotData::Points(vec![
        PointDatum {
            key: "north".into(),
            label: "North".into(),
            x: 1.,
            y: 3.,
            size: 1.,
        },
        PointDatum {
            key: "south".into(),
            label: "South".into(),
            x: 4.,
            y: 2.,
            size: 4.,
        },
        PointDatum {
            key: "west".into(),
            label: "West".into(),
            x: 2.,
            y: 5.,
            size: 9.,
        },
        PointDatum {
            key: "east".into(),
            label: "East".into(),
            x: 5.,
            y: 4.,
            size: 3.,
        },
    ])
}
fn sample_data() -> PlotData {
    points()
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = controls::chart_palette(&theme);
    scatter_chart("reference-scatter-chart-remove-tooltip")
        .theme(theme)
        .name("Scatter chart")
        .chart_theme(palette)
        .data(sample_data())
        .frame(ChartFrame {
            width: 640.,
            height: 270.,
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
