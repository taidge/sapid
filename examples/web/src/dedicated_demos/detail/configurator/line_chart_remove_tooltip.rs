use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Line;

    controls::fruit_sales_chart(&theme, "line-chart-remove-tooltip", kind).default_options(
        CartesianOptions {
            presentation: ChartPresentation {
                tooltip: false,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}
