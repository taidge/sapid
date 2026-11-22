use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let align = scope.cage(Some("stretch".to_owned()));
        let justify = scope.cage(Some("center".to_owned()));
        let gap = scope.cage(Some("md".to_owned()));
        let palette = theme.clone();
        let layout = scope.bond(move || Style {
            align: Some(match align.get().as_deref() {
                Some("center") => Align::Center,
                Some("start") => Align::Start,
                Some("end") => Align::End,
                _ => Align::Stretch,
            }),
            justify: Some(match justify.get().as_deref() {
                Some("start") => Justify::Start,
                Some("end") => Justify::End,
                Some("between") => Justify::Between,
                Some("around") => Justify::Around,
                Some("evenly") => Justify::Evenly,
                _ => Justify::Center,
            }),
            ..Style::new()
                .height(Length::Px(300.))
                .width(Length::Percent(100.))
                .gap(palette.spacing(controls::size_value(gap.get().as_deref().unwrap_or("md"))))
        });
        let stack = sapid::stack()
            .theme(theme.clone())
            .class("site-stack-example")
            .style(layout)
            .fill(Vec::from(["Plan", "Design review", "Deliver"].map(
                |label| {
                    sapid::card()
                        .theme(theme.clone())
                        .style(Style::new().padding(Edges::all(8.)))
                        .fill(sapid::text().theme(theme.clone()).content(label))
                },
            )));
        view()
            .class("site-configurator")
            .fill(controls::playground(
                Box::new(stack),
                vec![
                    controls::select(
                        "stack-align",
                        "Align",
                        align,
                        &[
                            ("stretch", "Stretch"),
                            ("center", "Center"),
                            ("start", "Start"),
                            ("end", "End"),
                        ],
                        &theme,
                    ),
                    controls::select(
                        "stack-justify",
                        "Justify",
                        justify,
                        &[
                            ("center", "Center"),
                            ("start", "Start"),
                            ("end", "End"),
                            ("between", "Space between"),
                            ("around", "Space around"),
                            ("evenly", "Space evenly"),
                        ],
                        &theme,
                    ),
                    controls::scale_slider("stack-gap", "Gap", gap, &theme),
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

    pub fn select(
        key: &str,
        label: &str,
        value: Cage<Option<String>>,
        options: &'static [(&'static str, &'static str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let field_key = format!("control-{key}");
        let select_key = format!("control-select-{key}");
        Box::new(
            sapid::field(field_key)
                .theme(theme.clone())
                .label(label.to_owned())
                .control(move |field| {
                    let mut select = sapid::native_select(select_key.clone()).field(field);
                    for (value, label) in options {
                        select = select.option(sapid::select_option(*value, *label));
                    }
                    select.value(value).on_value_change(move |next| {
                        if next.is_some() {
                            value.revise(|current| *current = next)
                        }
                    })
                }),
        )
    }

    pub fn scale_slider(
        key: &'static str,
        label: &'static str,
        value: Cage<Option<String>>,
        theme: &Theme,
    ) -> Box<dyn Widget> {
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
                    .content(label),
                sapid::slider(key)
                    .theme(theme.clone())
                    .name(label)
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
