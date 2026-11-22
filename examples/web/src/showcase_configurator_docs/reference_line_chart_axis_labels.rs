use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::reference_sales_chart(&theme, "line-chart-20", CartesianKind::Line).default_options(
        CartesianOptions {
            kind: CartesianKind::Line,
            domain: Some(ChartDomain::new(0., 3600.).unwrap()),
            x_axis_title: Some("Date".into()),
            y_axis_title: Some("Amount".into()),
            presentation: ChartPresentation {
                curve: ChartCurve::Monotone,
                fill_opacity: 0.2,
                dot_size: 4.,
                edge_categories: true,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}

#[allow(unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

    pub fn fruit_sales_chart(
        theme: &Theme,
        key: &'static str,
        kind: sapid::charts::CartesianKind,
    ) -> sapid::charts::Chart {
        use sapid::charts::*;
        let data = fruit_sales_data(kind);
        let mut palette = ChartTheme::from_theme(theme);
        palette.series[..3].copy_from_slice(&[
            Color(76, 110, 245, 255),
            Color(34, 139, 230, 255),
            Color(18, 184, 134, 255),
        ]);
        chart(key)
            .theme(theme.clone())
            .kind(kind)
            .chart_theme(palette)
            .frame(ChartFrame {
                width: 640.,
                height: 300.,
            })
            .data(data)
            .data_table(false)
            .legend(false)
            .navigation(false)
    }
    pub fn fruit_sales_data(kind: sapid::charts::CartesianKind) -> sapid::charts::ChartData {
        use sapid::charts::*;
        let categories = ["Mar 22", "Mar 23", "Mar 24", "Mar 25", "Mar 26"]
            .into_iter()
            .map(|v| ChartCategory::new(v.to_ascii_lowercase().replace(' ', "-"), v))
            .collect();
        let mut series = vec![
            ChartSeries::new(
                "apples",
                "Apples",
                [2890., 2756., 3322., 3470., 3129.].map(Some),
            ),
            ChartSeries::new(
                "oranges",
                "Oranges",
                [2338., 2103., 986., 2108., 1726.].map(Some),
            ),
            ChartSeries::new(
                "tomatoes",
                "Tomatoes",
                [2452., 2402., 1821., 2809., 2290.].map(Some),
            ),
        ];
        if kind == CartesianKind::Composite {
            series[1].kind = ChartSeriesKind::Area;
            series[2].kind = ChartSeriesKind::Bar;
        }
        ChartData { categories, series }
    }

    pub fn reference_sales_data(kind: sapid::charts::CartesianKind) -> sapid::charts::ChartData {
        use sapid::charts::*;
        if kind == CartesianKind::Bar {
            return ChartData {
                categories: ["January", "February", "March", "April", "May", "June"]
                    .map(|month| ChartCategory::new(month.to_lowercase(), month))
                    .into(),
                series: vec![
                    ChartSeries::new(
                        "smartphones",
                        "Smartphones",
                        [1200., 1900., 400., 1000., 800., 750.].map(Some),
                    ),
                    ChartSeries::new(
                        "laptops",
                        "Laptops",
                        [900., 1200., 1000., 200., 1400., 600.].map(Some),
                    ),
                    ChartSeries::new(
                        "tablets",
                        "Tablets",
                        [200., 400., 200., 800., 1200., 1000.].map(Some),
                    ),
                ],
            };
        }
        let mut data = fruit_sales_data(kind);
        if kind == CartesianKind::Composite {
            let mut oranges = ChartSeries::new(
                "oranges",
                "Oranges",
                [723., 1887., 986., 987., 403.].map(Some),
            );
            oranges.kind = ChartSeriesKind::Area;
            let apples = ChartSeries::new(
                "apples",
                "Apples",
                [2301., 2181., 1821., 2764., 1821.].map(Some),
            );
            let tomatoes = data.series.pop().unwrap();
            data.series = vec![tomatoes, oranges, apples];
        }
        data
    }
    pub fn reference_sales_palette(
        theme: &Theme,
        kind: sapid::charts::CartesianKind,
    ) -> sapid::charts::ChartTheme {
        use sapid::charts::*;
        let mut palette = ChartTheme::from_theme(theme);
        palette.series[..3].copy_from_slice(&match kind {
            CartesianKind::Composite => [
                Color(18, 120, 255, 51),
                Color(230, 119, 0, 255),
                Color(224, 49, 49, 255),
            ],
            CartesianKind::Bar => [
                Color(121, 80, 242, 255),
                Color(34, 139, 230, 255),
                Color(18, 184, 134, 255),
            ],
            _ => [
                Color(76, 110, 245, 255),
                Color(34, 139, 230, 255),
                Color(18, 184, 134, 255),
            ],
        });
        palette
    }
    pub fn reference_sales_chart(
        theme: &Theme,
        key: &'static str,
        kind: sapid::charts::CartesianKind,
    ) -> sapid::charts::Chart {
        fruit_sales_chart(theme, key, kind)
            .data(reference_sales_data(kind))
            .chart_theme(reference_sales_palette(theme, kind))
    }
}
