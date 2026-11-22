use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Composite;
    let mut data = controls::reference_sales_data(kind);
    data.series.retain(|series| series.key == "apples");
    let mut palette = controls::reference_sales_palette(&theme,CartesianKind::Composite);
    palette.series[0] = if theme.scheme == sapid::ColorScheme::Dark {
        Color(169, 227, 75, 255)
    } else {
        Color(232, 89, 12, 255)
    };
    controls::reference_sales_chart(&theme, "composite-chart-18", CartesianKind::Composite)
        .data(data)
        .chart_theme(palette)
        .default_options(CartesianOptions {
            kind: CartesianKind::Composite,
            domain: Some(ChartDomain::new(0., 3000.).unwrap()),
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
