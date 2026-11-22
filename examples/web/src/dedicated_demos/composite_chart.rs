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
        "composite-chart-overview",
        CartesianKind::Composite,
        ChartData {
            categories,
            series: vec![
                ChartSeries::new(
                    "apples",
                    "Apples",
                    [2301.0, 2181.0, 1821.0, 2764.0, 1821.0].map(Some),
                )
                .kind(ChartSeriesKind::Line),
                ChartSeries::new(
                    "oranges",
                    "Oranges",
                    [723.0, 1887.0, 986.0, 987.0, 403.0].map(Some),
                )
                .kind(ChartSeriesKind::Area),
                ChartSeries::new(
                    "tomatoes",
                    "Tomatoes",
                    [2452.0, 2402.0, 1821.0, 2809.0, 2290.0].map(Some),
                )
                .kind(ChartSeriesKind::Bar),
            ],
        },
    )
}
