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

#[allow(unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

    pub fn playground(preview: Box<dyn Widget>, controls: Vec<Box<dyn Widget>>) -> Box<dyn Widget> {
        Box::new(
            view()
                .class("site-button-playground")
                .style(
                    Style::new()
                        .display(Display::Flex)
                        .flex_wrap(FlexWrap::Wrap)
                        .gap(24.0),
                )
                .fill((
                    col()
                        .class("site-playground-preview")
                        .style(Style {
                            grow: Some(1.0),
                            align: Some(Align::Center),
                            justify: Some(Justify::Center),
                            ..Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .min_width(Length::Px(180.0))
                        })
                        .fill(preview),
                    col()
                        .class("site-playground-controls")
                        .style(
                            Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .width(Length::Px(240.0))
                                .max_width(Limit::Percent(100.0)),
                        )
                        .fill(controls),
                )),
        )
    }

    pub fn segmented_choice(
        key: &'static str,
        label: &'static str,
        value: Cage<String>,
        options: &'static [(&str, &str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        Box::new(
            sapid::segmented_control(key)
                .theme(theme.clone())
                .size(Size::Small)
                .label(label)
                .full_width(true)
                .value(value)
                .segments(
                    options
                        .iter()
                        .map(|&(key, caption)| sapid::segment(key).label(caption)),
                )
                .on_value_change(move |next| value.revise(|current| *current = next)),
        )
    }

    pub fn switch(label: &str, value: Cage<bool>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::switch()
                .theme(theme.clone())
                .size(Size::Small)
                .with_thumb_indicator(true)
                .label(label.to_owned())
                .checked(value)
                .on_checked_change(move |next| value.revise(|current| *current = next)),
        )
    }

    pub fn chart_axes(value: &str) -> sapid::charts::ChartAxes {
        use sapid::charts::ChartAxes::*;
        match value {
            "x" => X,
            "y" => Y,
            "xy" => Both,
            _ => None,
        }
    }
    pub fn chart_palette(theme: &Theme) -> sapid::charts::ChartTheme {
        let mut palette = sapid::charts::ChartTheme::from_theme(theme);
        palette.series[..4].copy_from_slice(&[
            Color(76, 110, 245, 255),
            Color(250, 176, 5, 255),
            Color(18, 184, 134, 255),
            Color(134, 142, 150, 255),
        ]);
        palette
    }
}
