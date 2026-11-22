use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Bar;

    controls::fruit_sales_chart(&theme, "bar-chart-legend", kind)
        .legend(true)
        .legend_above_plot(true)
}
