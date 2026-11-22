use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Composite;

    controls::fruit_sales_chart(&theme, "composite-chart-dashed-lines", kind).default_options(
        CartesianOptions {
            presentation: ChartPresentation {
                dashed_series: true,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}
