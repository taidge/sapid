use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Line;

    controls::fruit_sales_chart(&theme, "line-chart-points-labels", kind).default_options(
        CartesianOptions {
            presentation: ChartPresentation {
                point_labels: true,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}
