use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let dark = theme.scheme == sapid::ColorScheme::Dark;
    let mut palette = controls::reference_sales_palette(&theme,CartesianKind::Composite);
    palette.grid = if dark {
        Color(255, 255, 255, 38)
    } else {
        Color(0, 0, 0, 38)
    };
    controls::reference_sales_chart(&theme, "composite-chart-19", CartesianKind::Composite)
        .chart_theme(palette)
        .default_options(CartesianOptions {
            kind: CartesianKind::Composite,
            domain: Some(ChartDomain::new(0., 3000.).unwrap()),
            presentation: ChartPresentation {
                curve: ChartCurve::Monotone,
                max_bar_width: Some(30.),
                fill_opacity: 0.2,
                dot_size: 4.,
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
