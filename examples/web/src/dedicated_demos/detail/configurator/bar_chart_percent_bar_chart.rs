use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Bar;
    let mut data = controls::fruit_sales_data(kind);
    for index in 0..data.categories.len() {
        let total: f64 = data.series.iter().filter_map(|s| s.values[index]).sum();
        for series in &mut data.series {
            series.values[index] = series.values[index].map(|v| v * 100. / total);
        }
    }
    controls::fruit_sales_chart(&theme, "bar-chart-percent-bar-chart", kind)
        .data(data)
        .default_options(CartesianOptions {
            stacked: true,
            domain: Some(ChartDomain::new(0., 100.).unwrap()),
            ..Default::default()
        })
}
