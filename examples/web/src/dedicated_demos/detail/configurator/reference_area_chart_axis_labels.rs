use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::reference_sales_chart(&theme, "area-chart-0", CartesianKind::Area).default_options(
        CartesianOptions {
            kind: CartesianKind::Area,
            stacked: true,
            domain: Some(ChartDomain::new(0., 10000.).unwrap()),
            x_axis_title: Some("Date".into()),
            y_axis_title: Some("Amount".into()),
            presentation: ChartPresentation {
                curve: ChartCurve::Monotone,
                fill_opacity: 0.2,
                dot_size: 4.,
                edge_categories: true,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}
