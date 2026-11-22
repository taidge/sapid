use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Area;
    let mut data = controls::reference_sales_data(kind);
    data.series.truncate(1);
    let mut palette = controls::reference_sales_palette(&theme,CartesianKind::Area);
    palette.series[0] = if theme.scheme == sapid::ColorScheme::Dark {
        Color(169, 227, 75, 255)
    } else {
        Color(232, 89, 12, 255)
    };
    controls::reference_sales_chart(&theme, "area-chart-5", CartesianKind::Area)
        .data(data)
        .chart_theme(palette)
        .default_options(CartesianOptions {
            kind: CartesianKind::Area,
            domain: Some(ChartDomain::new(0., 3600.).unwrap()),
            presentation: ChartPresentation {
                curve: ChartCurve::Monotone,
                fill_opacity: 0.2,
                dot_size: 4.,
                edge_categories: true,
                ..Default::default()
            },
            ..Default::default()
        })
}
