use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let dark = theme.scheme == sapid::ColorScheme::Dark;
    let mut palette = controls::reference_sales_palette(&theme,CartesianKind::Bar);
    palette.grid = if dark {
        Color(255, 255, 255, 38)
    } else {
        Color(0, 0, 0, 38)
    };
    controls::reference_sales_chart(&theme, "bar-chart-12", CartesianKind::Bar)
        .chart_theme(palette)
        .default_options(CartesianOptions {
            kind: CartesianKind::Bar,
            domain: Some(ChartDomain::new(0., 2000.).unwrap()),
            presentation: ChartPresentation {
                curve: ChartCurve::Monotone,
                fill_opacity: 0.2,
                dot_size: 4.,
                dots: false,
                axis_text_color: Some(if dark {
                    Color(201, 201, 201, 255)
                } else {
                    Color(73, 80, 87, 255)
                }),
                ..Default::default()
            },
            ..Default::default()
        })
}
