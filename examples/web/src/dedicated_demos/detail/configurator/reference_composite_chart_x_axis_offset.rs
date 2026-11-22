use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::reference_sales_chart(&theme, "composite-chart-14", CartesianKind::Composite)
        .default_options(CartesianOptions {
            kind: CartesianKind::Composite,
            domain: Some(ChartDomain::new(0., 3000.).unwrap()),
            presentation: ChartPresentation {
                curve: ChartCurve::Monotone,
                max_bar_width: Some(30.),
                fill_opacity: 0.2,
                dot_size: 4.,
                x_inset: 30.,
                ..Default::default()
            },
            ..Default::default()
        })
}
