use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Bar;

    controls::fruit_sales_chart(&theme, "bar-chart-reference-areas", kind).reference_areas([
        ChartReferenceArea::new()
            .categories("mar-23", "mar-25")
            .label("Campaign"),
    ])
}
