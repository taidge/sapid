use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let current = scope.cage(0usize);
        let align = scope.cage("center".to_owned());
        let looping = scope.cage(true);
        let free = scope.cage(false);
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(align.get(), looping.get(), free.get())]))
            .key(Clone::clone)
            .children(move |entry| {
                let (align, looping, free) = entry.value.get();
                let blue = palette.accent;
                sapid::carousel()
                    .name("Embla options slides")
                    .theme(palette.clone())
                    .count(5usize)
                    .viewport_height(200.)
                    .slide_fraction(0.7)
                    .overscan(2)
                    .slide_gap(16.)
                    .control_size(26.)
                    .controls_offset(12.)
                    .with_indicators(true)
                    .slide_align(match align.as_str() {
                        "start" => Align::Start,
                        "end" => Align::End,
                        _ => Align::Center,
                    })
                    .looping(looping)
                    .drag_free(free)
                    .current(current)
                    .on_current_change(move |next| current.revise(|old| *old = next))
                    .render_slide(move |index| {
                        view()
                            .class("site-carousel-number")
                            .style(Style {
                                background: Some(blue),
                                color: Some(glory::style::Color(255, 255, 255, 255)),
                                ..Style::new()
                            })
                            .fill(text().content((index + 1).to_string()))
                    })
            });
        view()
            .class("site-configurator site-carousel-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::segmented_choice(
                        "carousel-align",
                        "Align",
                        align,
                        &[("start", "Start"), ("center", "Center"), ("end", "End")],
                        &theme,
                    ),
                    controls::switch("Loop", looping, &theme),
                    controls::switch("Drag free", free, &theme),
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
}
