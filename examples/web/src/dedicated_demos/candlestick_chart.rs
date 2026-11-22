use glory::prelude::*;
use sapid::charts::*;
fn candles() -> PlotData {
    PlotData::Candles(vec![
        CandleDatum {
            key: "mon".into(),
            label: "Monday".into(),
            open: 3.,
            high: 8.,
            low: 1.,
            close: 6.,
        },
        CandleDatum {
            key: "tue".into(),
            label: "Tuesday".into(),
            open: 6.,
            high: 9.,
            low: 2.,
            close: 4.,
        },
        CandleDatum {
            key: "wed".into(),
            label: "Wednesday".into(),
            open: 4.,
            high: 7.,
            low: 3.,
            close: 5.,
        },
        CandleDatum {
            key: "thu".into(),
            label: "Thursday".into(),
            open: 5.,
            high: 10.,
            low: 4.,
            close: 9.,
        },
    ])
}
fn sample_data() -> PlotData {
    candles()
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let ticks = scope.cage("y".to_string());
        let grid = scope.cage("x".to_string());
        let x = scope.cage(true);
        let y = scope.cage(true);
        let tooltip = scope.cage(true);
        let options = scope.bond(move || PlotOptions {
            candle_axes: Some(CandleAxes {
                ticks: controls::chart_axes(&ticks.get()),
                grid: controls::chart_axes(&grid.get()),
                x: x.get(),
                y: y.get(),
            }),
            tooltip: tooltip.get(),
            ..Default::default()
        });
        let preview = candlestick_chart("candlestick-chart-overview")
            .theme(theme.clone())
            .data(sample_data())
            .frame(ChartFrame {
                width: 520.,
                height: 300.,
            })
            .chart_theme(controls::chart_palette(&theme))
            .options(options)
            .legend(false)
            .navigation(false)
            .data_table(false);
        let axes = &[("x", "x"), ("y", "y"), ("xy", "xy"), ("none", "none")];
        let settings = vec![
            controls::segmented_choice("candle-ticks", "Tick line", ticks, axes, &theme),
            controls::segmented_choice("candle-grid", "Grid axis", grid, axes, &theme),
            controls::switch("With xaxis", x, &theme),
            controls::switch("With yaxis", y, &theme),
            controls::switch("With tooltip", tooltip, &theme),
        ];
        view()
            .class("site-configurator site-chart-configurator site-compact-plot")
            .fill(controls::playground(Box::new(preview), settings))
            .show_in(scope)
    })
}
