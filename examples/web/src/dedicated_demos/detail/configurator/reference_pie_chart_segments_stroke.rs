use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut palette = controls::chart_palette(&theme);
    let card = match theme.scheme {
        sapid::ColorScheme::Light => Color(241, 243, 245, 255),
        sapid::ColorScheme::Dark => Color(44, 46, 51, 255),
    };
    palette.grid = card;
    let chart = pie_chart("reference-pie-chart-segments-stroke")
        .theme(theme)
        .name("Share by country")
        .chart_theme(palette)
        .data(controls::segment_example_data())
        .frame(ChartFrame {
            width: 320.,
            height: 320.,
        })
        .default_options(PlotOptions {
            angles: Some(PlotAngles {
                start: 0.,
                sweep: 360.,
            }),
            counterclockwise: true,
            segment_stroke: Some(2.),
            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(184.))
                .max_width(Limit::Percent(100.)),
        )
        .legend(false)
        .navigation(false)
        .data_table(false);
    view()
        .style(Style {
            background: Some(card),
            ..Style::new().padding(Edges::all(16.)).radius(8.)
        })
        .fill(chart)
}
