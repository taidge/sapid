use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let leader = scope.cage(true);
        let position = scope.cage("outside".to_owned());
        let value = scope.cage("value".to_owned());
        let options = scope.bond(move || PlotOptions {
            angles: Some(PlotAngles {
                start: 0.,
                sweep: 360.,
            }),
            counterclockwise: true,
            segment_labels: Some(SegmentLabels {
                position: if position.get() == "inside" {
                    SegmentLabelPosition::Inside
                } else {
                    SegmentLabelPosition::Outside
                },
                value: match value.get().as_str() {
                    "percent" => SegmentLabelValue::Percent,
                    "name" => SegmentLabelValue::Name,
                    _ => SegmentLabelValue::Value,
                },
                leader: leader.get(),
            }),
            ..Default::default()
        });
        let palette = controls::chart_palette(&theme);
        let preview = plot("pie-chart-segments-labels", PlotKind::Pie)
            .theme(theme.clone())
            .chart_theme(palette)
            .options(options)
            .frame(ChartFrame {
                width: 320.,
                height: 320.,
            })
            .data(controls::segment_example_data())
            .legend(false)
            .data_table(false)
            .navigation(false)
            .style(scope.bond(move || {
                Style::new()
                    .width(Length::Px(300.))
                    .max_width(Limit::Percent(100.))
            }));
        view()
            .class("site-configurator site-chart-configurator site-compact-plot")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("With labels line", leader, &theme),
                    controls::segmented_choice(
                        "pie-chart-segments-labels-position",
                        "Labels position",
                        position,
                        &[("inside", "inside"), ("outside", "outside")],
                        &theme,
                    ),
                    controls::segmented_choice(
                        "pie-chart-segments-labels-value",
                        "Labels type",
                        value,
                        &[("value", "value"), ("percent", "percent"), ("name", "name")],
                        &theme,
                    ),
                ],
            ))
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

    pub fn segment_example_data() -> sapid::charts::PlotData {
        use sapid::charts::*;
        PlotData::values(
            [
                ("usa", "USA", 400.),
                ("india", "India", 300.),
                ("japan", "Japan", 100.),
                ("other", "Other", 200.),
            ]
            .map(|(key, label, value)| ValueDatum::new(key, label, value)),
        )
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
