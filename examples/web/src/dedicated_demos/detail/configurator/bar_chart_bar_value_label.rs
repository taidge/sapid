use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Bar;

    controls::fruit_sales_chart(&theme, "bar-chart-bar-value-label", kind).default_options(
        CartesianOptions {
            presentation: ChartPresentation {
                point_labels: true,
                dots: false,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}
