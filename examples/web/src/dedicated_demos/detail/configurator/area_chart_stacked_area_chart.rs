use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Area;

    controls::fruit_sales_chart(&theme, "area-chart-stacked-area-chart", kind).default_options(
        CartesianOptions {
            stacked: true,
            ..Default::default()
        },
    )
}
