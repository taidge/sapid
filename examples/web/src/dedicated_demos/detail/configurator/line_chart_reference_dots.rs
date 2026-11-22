use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Line;

    controls::fruit_sales_chart(&theme, "line-chart-reference-dots", kind).reference_dots([
        ChartReferenceDot::new("mar-25", 3470.)
            .label("Peak")
            .color(Color(250, 82, 82, 255)),
    ])
}
