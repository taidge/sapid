use glory::prelude::*;
use sapid::{DateLocale, HourCycle, LocalTime, Size, TimeFormat, TimeGridSpec, TimePrecision};

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(None::<LocalTime>);
        let cycle = scope.cage("24h".to_owned());
        let seconds = scope.cage(false);
        let size = scope.cage(1.);
        let radius = scope.cage(1.);
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                cycle.get(),
                seconds.get(),
                size.get() as usize,
                radius.get() as usize,
            )]
        }))
        .key(Clone::clone)
        .children(move |entry| {
            let (cycle, seconds, index, radius) = entry.value.get();
            let mut theme = palette.clone();
            theme.radius = [2., 4., 8., 16., 32.][radius];
            sapid::time_grid("appointment-time")
                .theme(theme)
                .size(
                    [
                        Size::ExtraSmall,
                        Size::Small,
                        Size::Medium,
                        Size::Large,
                        Size::ExtraLarge,
                    ][index],
                )
                .name("Appointment time")
                .spec(
                    TimeGridSpec::new(
                        LocalTime::new(10, 0, 0).unwrap(),
                        LocalTime::new(21, 0, 0).unwrap(),
                        3600,
                        3,
                    )
                    .unwrap(),
                )
                .format(TimeFormat::new(
                    if cycle == "12h" {
                        HourCycle::H12
                    } else {
                        HourCycle::H24
                    },
                    if seconds {
                        TimePrecision::Second
                    } else {
                        TimePrecision::Minute
                    },
                    DateLocale::En,
                ))
                .value(selected)
                .on_value_change(move |next| selected.revise(|old| *old = next))
        });
        view()
            .class("site-configurator site-time-grid-configurator")
            .fill(controls::playground(
                Box::new(
                    sapid::stack().theme(theme.clone()).gap(8.).fill((
                        preview,
                        sapid::text()
                            .theme(theme.clone())
                            .content(selected.map(|value| {
                                value.map_or_else(String::new, |time| format!("Selected: {time}"))
                            })),
                    )),
                ),
                vec![
                    controls::segmented_choice(
                        "time-grid-format",
                        "Format",
                        cycle,
                        &[("12h", "12h"), ("24h", "24h")],
                        &theme,
                    ),
                    controls::switch("With seconds", seconds, &theme),
                    controls::field_scale("time-grid-size".into(), "Size", size, &theme),
                    controls::field_scale("time-grid-radius".into(), "Radius", radius, &theme),
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

    pub fn field_scale(
        key: String,
        label: &'static str,
        step: Cage<f64>,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        const NAMES: [&str; 5] = ["xs", "sm", "md", "lg", "xl"];
        Box::new(
            sapid::stack().theme(theme.clone()).gap(4.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content(label),
                sapid::slider(key)
                    .theme(theme.clone())
                    .name(label)
                    .min(0.)
                    .max(4.)
                    .step(1.)
                    .marks((0..5).map(|i| sapid::SliderMark::unlabeled(f64::from(i))))
                    .value_text(step.map(|i| Some(NAMES[*i as usize].to_owned())))
                    .value(step)
                    .on_value_change(move |next| step.revise(|value| *value = next)),
            )),
        )
    }
}
