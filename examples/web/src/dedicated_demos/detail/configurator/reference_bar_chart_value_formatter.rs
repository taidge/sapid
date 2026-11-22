use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::reference_sales_chart(&theme, "bar-chart-10", CartesianKind::Bar).default_options(
        CartesianOptions {
            kind: CartesianKind::Bar,
            domain: Some(ChartDomain::new(0., 2000.).unwrap()),
            presentation: ChartPresentation {
                curve: ChartCurve::Monotone,
                fill_opacity: 0.2,
                dot_size: 4.,
                dots: false,
                group_digits: true,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}
