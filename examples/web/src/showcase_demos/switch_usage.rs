use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let checked = scope.cage(true);
        let label = scope.cage("Notifications".to_owned());
        let readonly = scope.cage(false);
        let disabled = scope.cage(false);
        let size = scope.cage(Some("md".to_owned()));
        let marks = scope.cage(true);
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(size.get(), marks.get())]))
            .key(Clone::clone)
            .children(move |_| {
                sapid::switch()
                    .theme(palette.clone())
                    .size(controls::size_value(size.get().as_deref().unwrap_or("md")))
                    .label(label)
                    .with_marks(marks.get())
                    .checked(checked)
                    .readonly(readonly)
                    .enabled(disabled.map(|v| !*v))
                    .on_checked_change(move |next| checked.revise(|v| *v = next))
            });
        view()
            .class("site-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::text_field("Label", label, &theme),
                    controls::size_slider(size, &theme),
                    controls::switch("Show marks", marks, &theme),
                    controls::switch("Read only", readonly, &theme),
                    controls::switch("Disabled", disabled, &theme),
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
                .label(label.to_owned())
                .checked(value)
                .on_checked_change(move |next| value.revise(|current| *current = next)),
        )
    }

    pub fn text_field(label: &str, value: Cage<String>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::field(format!("control-{label}"))
                .theme(theme.clone())
                .label(label.to_owned())
                .control(move |field| {
                    sapid::text_field()
                        .field(field)
                        .placeholder("Enter prop value")
                        .value(value)
                        .on_value_change(move |next| value.revise(|current| *current = next))
                }),
        )
    }

    pub fn size_slider(value: Cage<Option<String>>, theme: &Theme) -> Box<dyn Widget> {
        let current = value.map(|current| match current.as_deref() {
            Some("xs") => 0.0,
            Some("sm") => 1.0,
            Some("lg") => 3.0,
            Some("xl") => 4.0,
            _ => 2.0,
        });
        let value_text = value.map(|current| {
            Some(
                match current.as_deref() {
                    Some("xs") => "Extra small",
                    Some("sm") => "Small",
                    Some("lg") => "Large",
                    Some("xl") => "Extra large",
                    _ => "Medium",
                }
                .to_owned(),
            )
        });
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.0).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content("Size"),
                sapid::slider("control-size")
                    .theme(theme.clone())
                    .name("Size")
                    .min(0.0)
                    .max(4.0)
                    .step(1.0)
                    .value(current)
                    .value_text(value_text)
                    .on_value_change(move |next| {
                        value.revise(|current| {
                            *current = Some(
                                match next.round() as u8 {
                                    0 => "xs",
                                    1 => "sm",
                                    3 => "lg",
                                    4 => "xl",
                                    _ => "md",
                                }
                                .to_owned(),
                            )
                        })
                    }),
            )),
        )
    }

    pub fn size_value(value: &str) -> Size {
        match value {
            "xs" => Size::ExtraSmall,
            "sm" => Size::Small,
            "lg" => Size::Large,
            "xl" => Size::ExtraLarge,
            _ => Size::Medium,
        }
    }
}
