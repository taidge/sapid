use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let categories = ["Mar 22", "Mar 23", "Mar 24", "Mar 25", "Mar 26"]
        .into_iter()
        .enumerate()
        .map(|(i, label)| ChartCategory::new(format!("c{i}"), label))
        .collect();
    controls::cartesian_playground(
        theme,
        "line-chart-overview",
        CartesianKind::Line,
        ChartData {
            categories,
            series: vec![
                ChartSeries::new(
                    "apples",
                    "Apples",
                    [2890.0, 2756.0, 3322.0, 3470.0, 3129.0].map(Some),
                ),
                ChartSeries::new(
                    "oranges",
                    "Oranges",
                    [2338.0, 2103.0, 986.0, 2108.0, 1726.0].map(Some),
                ),
                ChartSeries::new(
                    "tomatoes",
                    "Tomatoes",
                    [2452.0, 2402.0, 1821.0, 2809.0, 2290.0].map(Some),
                ),
            ],
        },
    )
}
