use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Line;

    controls::fruit_sales_chart(&theme, "line-chart-customize-dots", kind).default_options(
        CartesianOptions {
            presentation: ChartPresentation {
                dot_size: 14.,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}
