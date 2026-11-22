use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Bar;

    controls::fruit_sales_chart(&theme, "bar-chart-remove-tooltip", kind).default_options(
        CartesianOptions {
            presentation: ChartPresentation {
                tooltip: false,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}
