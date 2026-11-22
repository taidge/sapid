use glory::prelude::*;
use sapid::charts::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let row_names = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let column_names = [
        "6 AM", "8 AM", "10 AM", "12 PM", "2 PM", "4 PM", "6 PM", "8 PM",
    ];
    let observations = [
        [3, 14, 26, 30, 28, 22, 12, 5],
        [4, 16, 28, 32, 30, 24, 14, 6],
        [5, 18, 30, 35, 32, 26, 16, 8],
        [3, 15, 25, 29, 27, 21, 13, 7],
        [2, 12, 22, 26, 24, 18, 15, 10],
        [1, 4, 8, 12, 14, 10, 8, 6],
        [-1, 2, 5, 9, 10, 7, 5, 3],
    ];
    let rows: Vec<_> = row_names
        .iter()
        .enumerate()
        .map(|(i, label)| ChartCategory::new(format!("row-{i}"), *label))
        .collect();
    let columns: Vec<_> = column_names
        .iter()
        .enumerate()
        .map(|(i, label)| ChartCategory::new(format!("col-{i}"), *label))
        .collect();
    let mut cells = Vec::new();
    for (i, values) in observations.iter().enumerate() {
        for (j, &value) in values.iter().enumerate() {
            if value >= 0 {
                cells.push(MatrixCell {
                    key: format!("value-{i}-{j}"),
                    row: rows[i].key.clone(),
                    column: columns[j].key.clone(),
                    value: f64::from(value),
                });
            }
        }
    }
    let mut palette = ChartTheme::from_theme(&theme);
    palette.grid = theme.border;
    palette.series[0] = Color(178, 242, 187, 255);
    palette.series[1] = Color(43, 138, 62, 255);
    matrix_chart("reference-matrix-chart-legend")
        .theme(theme)
        .chart_theme(palette)
        .name("Contribution matrix")
        .data(PlotData::Matrix(MatrixData {
            rows,
            columns,
            cells,
        }))
        .frame(ChartFrame {
            width: 208.,
            height: 162.,
        })
        .default_options(PlotOptions {
            matrix_tiles: Some(MatrixTiles {
                size: 18.,
                gap: 2.,
                x_labels: true,
                y_labels: true,
                legend: true,
            }),
            tooltip: true,

            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(208.))
                .max_width(Limit::Percent(100.)),
        )
        .legend(false)
        .navigation(false)
        .data_table(false)
}
