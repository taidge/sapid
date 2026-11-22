use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::reference_sales_chart(&theme, "area-chart-3", CartesianKind::Area)
        .class("site-chart-rotated-labels")
        .default_options(CartesianOptions {
            kind: CartesianKind::Area,
            stacked: true,
            domain: Some(ChartDomain::new(0., 10000.).unwrap()),
            presentation: ChartPresentation {
                curve: ChartCurve::Monotone,
                fill_opacity: 0.2,
                dot_size: 4.,
                edge_categories: true,
                tick_lines: ChartAxes::Both,
                ..Default::default()
            },
            ..Default::default()
        })
}
