use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let angle = scope.cage(0.);
        let diameter = scope.cage(60.);
        let thumb = scope.cage(3.);
        let label = scope.cage(true);
        let preview = view()
            .style(diameter.map(|px| {
                Style::new()
                    .position(Position::Relative)
                    .width(Length::Px(*px))
                    .height(Length::Px(*px))
            }))
            .fill((
                sapid::angle_slider("rotation-angle")
                    .theme(theme.clone())
                    .name("Rotation angle")
                    .thumb_size(thumb)
                    .style(
                        diameter
                            .map(|px| Style::new().width(Length::Px(*px)).height(Length::Px(*px))),
                    )
                    .value(angle)
                    .on_value_change(move |next| angle.revise(|v| *v = next)),
                when(label, move || {
                    text()
                        .class("site-angle-value")
                        .style(Style {
                            display: Some(Display::Flex),
                            align: Some(Align::Center),
                            justify: Some(Justify::Center),
                            ..Style::new()
                                .position(Position::Absolute)
                                .left(Length::Px(0.))
                                .top(Length::Px(0.))
                                .width(Length::Percent(100.))
                                .height(Length::Percent(100.))
                                .pointer_events(glory::style::PointerEvents::None)
                        })
                        .content(angle.map(|v| format!("{v:.0}")))
                }),
            ));
        view()
            .class("site-configurator site-field-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::range("angle-size", "Size", diameter, 40., 240., 1., &theme),
                    controls::range("angle-thumb-size", "Thumb size", thumb, 1., 18., 1., &theme),
                    controls::switch("With label", label, &theme),
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

    pub fn range(
        key: &str,
        label: &str,
        value: Cage<f64>,
        min: f64,
        max: f64,
        step: f64,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let label = label.to_owned();
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content(value.map({
                        let label = label.clone();
                        move |v| {
                            let decimals = if step.fract() == 0. {
                                0
                            } else if (step * 10.).fract() == 0. {
                                1
                            } else {
                                2
                            };
                            format!("{label} · {v:.decimals$}")
                        }
                    })),
                sapid::slider(key.to_owned())
                    .theme(theme.clone())
                    .name(label)
                    .min(min)
                    .max(max)
                    .step(step)
                    .value(value)
                    .on_value_change(move |next| value.revise(|v| *v = next)),
            )),
        )
    }
}
