use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let categories = ["January", "February", "March", "April", "May", "June"]
        .into_iter()
        .enumerate()
        .map(|(i, label)| ChartCategory::new(format!("c{i}"), label))
        .collect();
    controls::cartesian_playground(
        theme,
        "bar-chart-overview",
        CartesianKind::Bar,
        ChartData {
            categories,
            series: vec![
                ChartSeries::new(
                    "smartphones",
                    "Smartphones",
                    [1200.0, 1900.0, 400.0, 1000.0, 800.0, 750.0].map(Some),
                ),
                ChartSeries::new(
                    "laptops",
                    "Laptops",
                    [900.0, 1200.0, 1000.0, 200.0, 1400.0, 600.0].map(Some),
                ),
                ChartSeries::new(
                    "tablets",
                    "Tablets",
                    [200.0, 400.0, 200.0, 800.0, 1200.0, 1000.0].map(Some),
                ),
            ],
        },
    )
}
