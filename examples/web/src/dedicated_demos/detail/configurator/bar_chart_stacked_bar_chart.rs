use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Bar;

    controls::fruit_sales_chart(&theme, "bar-chart-stacked-bar-chart", kind).default_options(
        CartesianOptions {
            stacked: true,
            ..Default::default()
        },
    )
}
