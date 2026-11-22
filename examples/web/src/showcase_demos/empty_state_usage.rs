use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let did_reset = scope.cage(false);

        let size = scope.cage(Some("md".to_owned()));
        let align = scope.cage(Some("center".to_owned()));
        let background = scope.cage(false);
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(size.get(),)]))
            .key(Clone::clone)
            .children(move |_| {
                let theme = palette.clone();
                let empty_theme = theme.clone();
                let ready_theme = theme.clone();
                sapid::stack().theme(theme.clone()).gap(12.).fill((
                    when(did_reset.map(|v| !*v), move || {
                        sapid::empty_state()
                            .theme(empty_theme.clone())
                            .size(controls::size_value(size.get().as_deref().unwrap_or("lg")))
                            .style(align.map(|s| Style {
                                align: Some(match s.as_deref() {
                                    Some("start") => Align::Start,
                                    Some("end") => Align::End,
                                    _ => Align::Center,
                                }),
                                ..Style::new()
                            }))
                            .illustration(
                                glory::ui::graphic()
                                    .drawing(sapid::Icon::Search.drawing(empty_theme.muted))
                                    .style(background.map({
                                        let soft = Color(241,243,245,255);
                                        move |on| Style {
                                            background: Some(if *on {
                                                soft
                                            } else {
                                                Color(0, 0, 0, 0)
                                            }),
                                            ..Style::new()
                                                .width(Length::Px(if *on {80.} else {48.}))
                                                .height(Length::Px(if *on {80.} else {48.}))
                                                .padding(Edges::all(if *on {18.} else {2.}))
                                                .radius(40.)
                                        }
                                    })),
                            )
                            .title("No results found")
                            .description(
                                "We couldn't find anything matching your search. Try adjusting your filters or searching with different keywords to see more results.",
                            )
                            .actions(sapid::button().theme(empty_theme.clone()).variant(sapid::ButtonVariant::Secondary).label("Reset filters").on_press(move || did_reset.revise(|v| *v = true)))
                    }),
                    when(did_reset, move || {
                        sapid::stack().theme(ready_theme.clone()).align(Align::Center).gap(12.).fill((sapid::text().theme(ready_theme.clone()).content("Filters reset."),sapid::button().theme(ready_theme.clone()).variant(sapid::ButtonVariant::Secondary).label("Restore filters").on_press(move || did_reset.revise(|v| *v=false))))
                    }),
                ))
            });
        view()
            .class("site-configurator site-display-configurator site-empty-state-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::size_slider(size, &theme),
                    controls::option_segments(
                        "empty_state-align",
                        "Align",
                        align,
                        &[("center", "Center"), ("start", "Left"), ("end", "Right")],
                        &theme,
                    ),
                    controls::switch("With indicator background", background, &theme),
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

    pub fn option_segments(
        key: &'static str,
        label: &'static str,
        value: Cage<Option<String>>,
        options: &'static [(&str, &str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        Box::new(
            sapid::stack().theme(theme.clone()).gap(4.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content(label),
                sapid::segmented_control(key)
                    .theme(theme.clone())
                    .size(Size::Small)
                    .name(label)
                    .full_width(true)
                    .value(value.map(|selected| selected.clone().unwrap_or_default()))
                    .segments(
                        options
                            .iter()
                            .map(|&(key, caption)| sapid::segment(key).label(caption)),
                    )
                    .on_value_change(move |next| value.revise(|current| *current = Some(next))),
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

    pub fn size_slider(value: Cage<Option<String>>, theme: &Theme) -> Box<dyn Widget> {
        scale_slider("control-size", "Size", value, theme)
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
