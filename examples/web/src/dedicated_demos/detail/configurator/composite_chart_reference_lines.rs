use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Composite;

    controls::fruit_sales_chart(&theme, "composite-chart-reference-lines", kind).reference_lines([
        ChartReferenceLine::value(2800.)
            .label("Sales target")
            .color(Color(250, 82, 82, 255)),
        ChartReferenceLine::category("mar-25").label("Report out"),
    ])
}
