use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Area;

    controls::fruit_sales_chart(&theme, "area-chart-dashed-area-line", kind).default_options(
        CartesianOptions {
            presentation: ChartPresentation {
                dashed_series: true,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}
