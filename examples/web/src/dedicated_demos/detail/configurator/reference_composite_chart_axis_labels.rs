use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::reference_sales_chart(&theme, "composite-chart-13", CartesianKind::Composite)
        .default_options(CartesianOptions {
            kind: CartesianKind::Composite,
            domain: Some(ChartDomain::new(0., 3000.).unwrap()),
            x_axis_title: Some("Date".into()),
            y_axis_title: Some("Amount".into()),
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
