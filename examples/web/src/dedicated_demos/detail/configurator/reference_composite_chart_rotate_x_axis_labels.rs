use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::reference_sales_chart(&theme, "composite-chart-16", CartesianKind::Composite)
        .class("site-chart-rotated-labels")
        .default_options(CartesianOptions {
            kind: CartesianKind::Composite,
            domain: Some(ChartDomain::new(0., 3000.).unwrap()),
            presentation: ChartPresentation {
                curve: ChartCurve::Monotone,
                max_bar_width: Some(30.),
                fill_opacity: 0.2,
                dot_size: 4.,
                tick_lines: ChartAxes::Both,
                ..Default::default()
            },
            ..Default::default()
        })
}
