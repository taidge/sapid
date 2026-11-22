use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(true);
        let aligned = scope.cage(false);
        let position = scope.cage("left".to_owned());
        let picked = scope.cage(Some("React".to_owned()));
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(active.get(), aligned.get(), position.get())]))
            .key(Clone::clone)
            .children(move |row| {
                let (with_icon, aligned, side) = row.value.get();
                let check = if !with_icon {
                    sapid::SelectCheckPosition::Hidden
                } else if side == "left" {
                    sapid::SelectCheckPosition::Left
                } else {
                    sapid::SelectCheckPosition::Right
                };
                let palette = palette.clone();
                view().class("site-framework-check-target").fill(
                    sapid::popover("framework-check-popup")
                        .theme(palette.clone())
                        .label("Select framework")
                        .show_close(false)
                        .open(true)
                        .width(sapid::PopoverWidth::Px(200.))
                        .content(move || {
                            view().class("site-framework-scroll-list").fill(
                                sapid::combobox("check-framework-option")
                                    .name("Framework")
                                    .theme(palette.clone())
                                    .inline_list(true)
                                    .open(true)
                                    .value(picked)
                                    .filter(sapid::ComboboxFilter::None)
                                    .check_position(check)
                                    .with_aligned_labels(aligned)
                                    .options(
                                        ["React", "Angular", "Svelte", "Vue"]
                                            .map(|v| sapid::select_option(v, v)),
                                    )
                                    .on_value_change(move |next| picked.revise(|v| *v = next)),
                            )
                        }),
                )
            });
        view()
            .class("site-configurator site-field-configurator site-framework-check")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("With check icon", active, &theme),
                    controls::switch("With aligned labels", aligned, &theme),
                    controls::segment_setting(
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
}
