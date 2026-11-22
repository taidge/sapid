use glory::prelude::*;
use sapid::charts::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let row_names = [
        "James", "Mary", "Robert", "Linda", "Michael", "Sarah", "David", "Emma",
    ];
    let column_names = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
        "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31",
    ];
    let observations = [
        [
            7, 10, 2, 10, 8, -1, -1, 6, 2, 8, 1, 3, 7, -1, 9, 10, -1, 1, 8, -1, -1, 5, 8, 2, 5, 6,
            -1, -1, 7, 7, 6,
        ],
        [
            3, 1, 8, 2, 2, -1, -1, 10, 1, 8, 8, 3, -1, -1, 9, 7, 5, 5, 3, -1, 3, 3, 1, 4, 5, 6, -1,
            -1, 9, 8, 2,
        ],
        [
            6, 3, 3, 5, 8, 7, -1, 9, 5, 2, 3, 5, -1, 1, 2, 8, 9, 1, 1, -1, -1, 9, 2, 4, 8, 10, -1,
            -1, 9, 3, 2,
        ],
        [
            5, 6, 9, 1, 2, -1, -1, 3, 3, 9, 8, 6, 6, -1, 2, 4, 6, 5, -1, 2, -1, 4, 6, -1, 5, 10,
            -1, -1, 1, 10, 1,
        ],
        [
            6, 9, 3, 4, 3, -1, -1, 8, 7, 2, 7, 7, 9, 5, 8, 4, 7, -1, 5, -1, -1, 7, 4, 7, 5, 2, -1,
            2, -1, 8, 9,
        ],
        [
            2, 5, 2, 2, 3, -1, 5, 9, 3, 10, 9, 5, 3, -1, 5, 9, 6, 9, 4, -1, 5, 9, 8, 2, 5, 5, 2,
            -1, 8, 2, 8,
        ],
        [
            3, 8, 2, 10, 1, 6, -1, 1, 5, 3, 1, 2, -1, -1, 1, 2, 1, 2, -1, -1, -1, 7, 7, 2, 5, 5,
            -1, -1, 5, 6, 2,
        ],
        [
            8, 9, 10, 4, 5, -1, -1, 1, 6, 6, 2, 6, -1, -1, 2, 4, 2, 9, 7, -1, 2, 9, 5, 6, 9, 9, -1,
            -1, 6, 1, 5,
        ],
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
    matrix_chart("matrix-chart-overview")
        .theme(theme)
        .chart_theme(palette)
        .name("Contribution matrix")
        .data(PlotData::Matrix(MatrixData {
            rows,
            columns,
            cells,
        }))
        .frame(ChartFrame {
            width: 668.,
            height: 160.,
        })
        .default_options(PlotOptions {
            matrix_tiles: Some(MatrixTiles {
                size: 18.,
                gap: 2.,
                x_labels: false,
                y_labels: true,
                legend: false,
            }),
            tooltip: true,

            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Px(668.))
                .max_width(Limit::Percent(100.)),
        )
        .legend(false)
        .navigation(false)
        .data_table(false)
}
