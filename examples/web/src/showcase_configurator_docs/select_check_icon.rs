use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::check_icon_playground(theme, "select_check_icon", false)
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

    pub fn segment_setting(
        key: &'static str,
        label: &'static str,
        value: Cage<String>,
        options: &'static [(&'static str, &'static str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        Box::new(
            sapid::segmented_control(key)
                .theme(theme.clone())
                .size(Size::Small)
                .class("site-variant-control")
                .label(label)
                .full_width(true)
                .value(value)
                .segments(
                    options
                        .iter()
                        .map(|&(key, label)| sapid::segment(key).label(label)),
                )
                .on_value_change(move |next| value.revise(|current| *current = next)),
        )
    }

    pub fn check_icon_playground(theme: Theme, key: &'static str, multiple: bool) -> impl Widget {
        widget(move |scope| {
            let active = scope.cage(true);
            let aligned = scope.cage(false);
            let position = scope.cage("left".to_owned());
            let selected = scope.cage(Some("React".to_owned()));
            let selected_many = scope.cage(vec!["React".to_owned()]);
            let opened = scope.cage(true);
            let palette = theme.clone();
            let preview =
                each(scope.bond(move || vec![(active.get(), aligned.get(), position.get())]))
                    .key(Clone::clone)
                    .children(move |item| -> Box<dyn Widget> {
                        let (active, aligned, position) = item.value.get();
                        let position = if !active {
                            sapid::SelectCheckPosition::Hidden
                        } else if position == "left" {
                            sapid::SelectCheckPosition::Left
                        } else {
                            sapid::SelectCheckPosition::Right
                        };
                        let palette = palette.clone();
                        Box::new(
                            sapid::field(format!("{key}-field"))
                                .theme(palette.clone())
                                .class("site-check-options-field")
                                .label("Control check icon")
                                .control(move |field| -> Box<dyn Widget> {
                                    let choices = ["React", "Angular", "Svelte", "Vue"]
                                        .map(|v| sapid::select_option(v, v));
                                    let style = Style::new()
                                        .width(Length::Percent(100.))
                                        .max_width(Limit::Px(340.));
                                    if multiple {
                                        Box::new(
                                            sapid::multi_select(key)
                                                .theme(palette.clone())
                                                .field(field)
                                                .placeholder("Pick value")
                                                .style(style)
                                                .options(choices)
                                                .check_position(position)
                                                .with_aligned_labels(aligned)
                                                .value(selected_many)
                                                .open(opened)
                                                .on_open_change(move |next| {
                                                    opened.revise(|v| *v = next)
                                                })
                                                .on_value_change(move |next| {
                                                    selected_many.revise(|v| *v = next)
                                                }),
                                        )
                                    } else {
                                        Box::new(
                                            sapid::select(key)
                                                .theme(palette.clone())
                                                .field(field)
                                                .placeholder("Pick value")
                                                .style(style)
                                                .options(choices)
                                                .check_position(position)
                                                .with_aligned_labels(aligned)
                                                .value(selected)
                                                .open(opened)
                                                .on_open_change(move |next| {
                                                    opened.revise(|v| *v = next)
                                                })
                                                .on_value_change(move |next| {
                                                    selected.revise(|v| *v = next)
                                                }),
                                        )
                                    }
                                }),
                        )
                    });
            view()
                .class("site-configurator site-field-configurator")
                .fill(playground(
                    Box::new(preview),
                    vec![
                        switch("With check icon", active, &theme),
                        switch("With aligned labels", aligned, &theme),
                        segment_setting(
                            "check-icon-side",
                            "Check icon position",
                            position,
                            &[("left", "Left"), ("right", "Right")],
                            &theme,
                        ),
                    ],
                ))
                .show_in(scope)
        })
    }
}
