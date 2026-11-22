use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut data = ChartData {
        categories: [
            "Mar 22", "Mar 23", "Mar 24", "Mar 25", "Mar 26", "Mar 27", "Mar 28", "Mar 29",
        ]
        .map(|d| ChartCategory::new(d.to_lowercase().replace(' ', "-"), d))
        .into(),
        series: vec![ChartSeries::new(
            "apples",
            "Apples",
            [50., 60., 40., 30., 0., 20., 20., 10.].map(Some),
        )],
    };
    data.series[0].kind = ChartSeriesKind::Area;
    controls::reference_sales_chart(&theme, "composite-chart-15", CartesianKind::Composite)
        .data(data)
        .default_options(CartesianOptions {
            kind: CartesianKind::Composite,
            domain: Some(ChartDomain::new(0., 100.).unwrap()),
            presentation: ChartPresentation {
                curve: ChartCurve::Monotone,
                max_bar_width: Some(30.),
                fill_opacity: 0.2,
                dot_size: 4.,
                ..Default::default()
            },
            ..Default::default()
        })
}
