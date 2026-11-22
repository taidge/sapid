use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Composite;

    controls::fruit_sales_chart(&theme, "composite-chart-x-and-y-axis-props", kind).default_options(
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
}
