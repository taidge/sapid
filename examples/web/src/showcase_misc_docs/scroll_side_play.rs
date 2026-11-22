use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let side = scope.cage(Some("right".to_owned()));
        let offset = scope.cage(true);
        let preview = {
            let theme = theme.clone();

            sapid::scroll_area("reading-scrollbar-side")
                .name("Reading with a positioned scrollbar")
                .theme(theme.clone())
                .scrollbars(sapid::Scrollbars::Always)
                .vertical_scrollbar_position(side.map(|v| {
                    if v.as_deref() == Some("left") {
                        sapid::VerticalScrollbarPosition::Left
                    } else {
                        sapid::VerticalScrollbarPosition::Right
                    }
                }))
                .offset_scrollbars(offset.map(|v| {
                    if *v {
                        sapid::ScrollbarOffset::Both
                    } else {
                        sapid::ScrollbarOffset::None
                    }
                }))
                .style(
                    Style::new()
                        .width(Length::Px(300.))
                        .max_width(Limit::Percent(100.))
                        .height(Length::Px(200.)),
                )
                .content(move |_| reading_article(theme.clone(), false))
        };
        view()
            .class("site-configurator site-display-configurator site-scroll-side-play-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::option_segments(
                        "scroll_side_play-side",
                        "Vertical scrollbar position",
                        side,
                        &[("left", "Left"), ("right", "Right")],
                        &theme,
                    ),
                    controls::switch("Offset scrollbars", offset, &theme),
                ],
            ))
            .show_in(scope)
    })
}

fn reading_article(theme: sapid::Theme, wide: bool) -> impl Widget {
    let width = if wide {
        Length::Px(600.)
    } else {
        Length::Percent(100.)
    };
    let heading = sapid::text()
        .theme(theme.clone())
        .weight(700.)
        .style(Style {
            font_size: Some(20.),
            ..Style::new()
        })
        .content("Charizard (Pokémon)");
    let attribution = sapid::text()
        .theme(theme.clone())
        .style(Style {
            color: Some(theme.muted),
            ..Style::new()
        })
        .content("Charizard description from Bulbapedia");
    let paragraphs = [
        "Charizard is an orange, two-legged Pokémon with a long tail tipped by a flame. Its wings have blue-green undersides, and its light-colored belly stretches from its chest to the base of its tail. Two horns rise from the back of its head. It flies in search of strong opponents, breathing fire that can melt rock and carrying itself high above mountain ridges. Each foot has three claws, and two pointed teeth show above its lower jaw. Its narrow arms contrast with a broad chest and powerful back legs. When it rests on the ground, its folded wings rise above its shoulders. The fire at the tip of its tail burns on land and in flight.",
            "Mega Charizard X has dark skin, a blue underside, and blue flames that trail from its mouth. Its broad shoulders carry a pair of spikes, and its wings and horns change shape. Its claws and brow grow larger as its flames intensify. Its horns curve upward, and small spines appear near the neck. The fire at the sides of its mouth glows blue. The lower edge of each wing follows a rounded outline, while the pointed tips reach beyond its shoulders. From the ground its dark silhouette looks quite different from its familiar orange form."
    ].into_iter().map(|paragraph| {
        sapid::text().theme(theme.clone()).style(Style {
            font_size: Some(14.),
            margin: Some(glory::style::Edges { top:16., right:0., bottom:0., left:0. }),
            ..Style::new()
        }).content(paragraph)
    }).collect::<Vec<_>>();
    sapid::stack()
        .theme(theme.clone())
        .gap(0.)
        .style(Style::new().width(width))
        .fill((
            heading,
            attribution,
            sapid::stack().theme(theme).gap(0.).fill(paragraphs),
        ))
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
}
