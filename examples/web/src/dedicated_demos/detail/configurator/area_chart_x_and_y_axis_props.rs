use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Area;

    controls::fruit_sales_chart(&theme, "area-chart-x-and-y-axis-props", kind).default_options(
        CartesianOptions {
            domain: Some(ChartDomain::new(0., 5000.).unwrap()),
            presentation: ChartPresentation {
                tick_lines: ChartAxes::Both,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}
