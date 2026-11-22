use glory::prelude::*;
use glory::style::FlexWrap;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let gap = scope.cage(Some("md".to_owned()));
        let justify = scope.cage(Some("start".to_owned()));
        let align = scope.cage(Some("start".to_owned()));
        let direction = scope.cage(Some("row".to_owned()));
        let wrap = scope.cage(Some("wrap".to_owned()));
        let palette = theme.clone();
        let layout = scope.bond(move || Style {
            justify: Some(controls::layout_justify(
                justify.get().as_deref().unwrap_or("start"),
            )),
            align: Some(controls::layout_align(
                align.get().as_deref().unwrap_or("start"),
            )),
            background: Some(Color(0, 0, 0, 77)),
            ..Style::new()
                .min_height(Length::Px(50.))
                .width(Length::Percent(100.))
                .flex_direction(
                    if direction
                        .get()
                        .as_deref()
                        .unwrap_or("row")
                        .starts_with("column")
                    {
                        FlexDirection::Column
                    } else {
                        FlexDirection::Row
                    },
                )
                .flex_wrap(if wrap.get().as_deref() == Some("nowrap") {
                    FlexWrap::NoWrap
                } else {
                    FlexWrap::Wrap
                })
                .gap(palette.spacing(controls::size_value(gap.get().as_deref().unwrap_or("md"))))
        });
        let classes = scope.bond(move || {
            format!(
                "site-flex-{} site-flex-{}",
                direction.get().as_deref().unwrap_or("row"),
                wrap.get().as_deref().unwrap_or("wrap")
            )
        });
        let preview = sapid::flex()
            .theme(theme.clone())
            .class(classes)
            .style(layout)
            .fill(Vec::from(["Button 1", "Button 2", "Button 3"].map(
                |label| sapid::button().theme(theme.clone()).label(label),
            )));
        view()
            .class("site-configurator site-layout-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::scale_slider("flex-gap", "Gap", gap, &theme),
                    controls::select(
                        "flex-justify",
                        "Justify",
                        justify,
                        &[
                            ("start", "Flex-start"),
                            ("center", "Center"),
                            ("end", "Flex-end"),
                        ],
                        &theme,
                    ),
                    controls::select(
                        "flex-align",
                        "Align",
                        align,
                        &[
                            ("start", "Flex-start"),
                            ("center", "Center"),
                            ("end", "Flex-end"),
                        ],
                        &theme,
                    ),
                    controls::select(
                        "flex-direction",
                        "Direction",
                        direction,
                        &[
                            ("row", "Row"),
                            ("column", "Column"),
                            ("row-reverse", "Row-reverse"),
                            ("column-reverse", "Column-reverse"),
                        ],
                        &theme,
                    ),
                    controls::select(
                        "flex-wrap",
                        "Wrap",
                        wrap,
                        &[
                            ("wrap", "Wrap"),
                            ("nowrap", "Nowrap"),
                            ("wrap-reverse", "Wrap-reverse"),
                        ],
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

    pub fn layout_justify(value: &str) -> Justify {
        match value {
            "center" => Justify::Center,
            "end" => Justify::End,
            "between" => Justify::Between,
            "around" => Justify::Around,
            "evenly" => Justify::Evenly,
            _ => Justify::Start,
        }
    }

    pub fn layout_align(value: &str) -> Align {
        match value {
            "center" => Align::Center,
            "end" => Align::End,
            "stretch" => Align::Stretch,
            _ => Align::Start,
        }
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
                    .marks((0..5).map(|i| sapid::SliderMark::unlabeled(f64::from(i))))
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
