use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let reverse = scope.cage(false);
        let hover = scope.cage(false);
        let axis = scope.cage(Some("horizontal".to_owned()));
        let repeat = scope.cage(4_f64);
        let duration = scope.cage(40000_f64);
        let gap = scope.cage(16_f64);
        let fade = scope.cage(true);
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                reverse.get(),
                axis.get(),
                repeat.get().to_bits(),
                gap.get().to_bits(),
            )]
        }))
        .key(Clone::clone)
        .children(move |_| {
            let theme = palette.clone();
            let vertical = axis.get().as_deref() == Some("vertical");
            let direction = match (vertical, reverse.get()) {
                (true, true) => sapid::MarqueeDirection::Down,
                (true, false) => sapid::MarqueeDirection::Up,
                (false, true) => sapid::MarqueeDirection::Right,
                _ => sapid::MarqueeDirection::Left,
            };
            let labels = [
                "Research", "Design", "Build", "Review", "Publish", "Measure", "Learn", "Improve",
            ];
            let message = labels.join(if vertical { "\n\n" } else { "     •     " });
            widget(move |scope| {
                let inside = scope.cage(false);
                sapid::marquee()
                    .theme(theme.clone())
                    .direction(direction)
                    .text(message.clone())
                    .visual_content(move || logo_strip(vertical, true))
                    .period_millis(duration.map(|v| *v as u32))
                    .gap(gap.get())
                    .repeat(repeat.get() as usize)
                    .class(fade.map(move |v| {
                        if !*v {
                            ""
                        } else if vertical {
                            "sapid-marquee-fade-vertical"
                        } else {
                            "sapid-marquee-fade-horizontal"
                        }
                        .to_owned()
                    }))
                    .paused(scope.bond(move || hover.get() && inside.get()))
                    .on(glory::render::EventKind::PointerWithin, move |event| {
                        if let glory::render::Event::PointerWithin { phase, .. } = event {
                            inside.revise(|v| *v = phase != glory::render::PointerPhase::Leave);
                        }
                    })
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(Limit::Px(400.))
                            .height(if vertical {
                                Length::Px(120.)
                            } else {
                                Length::Auto
                            }),
                    )
                    .show_in(scope)
            })
        });
        view()
            .class(
                "site-configurator site-display-configurator site-marquee-style-play-configurator",
            )
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("Reverse", reverse, &theme),
                    controls::switch("Pause on hover", hover, &theme),
                    controls::option_segments(
                        "marquee_style_play-axis",
                        "Orientation",
                        axis,
                        &[("horizontal", "Horizontal"), ("vertical", "Vertical")],
                        &theme,
                    ),
                    controls::range(
                        "marquee_style_play-repeat",
                        "Repeat",
                        repeat,
                        1.,
                        10.,
                        1.,
                        &theme,
                    ),
                    controls::range(
                        "marquee_style_play-duration",
                        "Duration",
                        duration,
                        5000.,
                        60000.,
                        1000.,
                        &theme,
                    ),
                    controls::range("marquee_style_play-gap", "Gap", gap, 0., 48., 4., &theme),
                    controls::switch("Fade edges", fade, &theme),
                ],
            ))
            .show_in(scope)
    })
}

pub(crate) fn logo_strip(vertical: bool, wide: bool) -> impl glory::ui::Widget {
    use glory::prelude::*;
    let size = if wide { 32. } else { 22. };
    let colors = [
        Color(34, 139, 230, 255),
        Color(21, 170, 191, 255),
        Color(18, 184, 134, 255),
        Color(64, 192, 87, 255),
        Color(130, 201, 30, 255),
        Color(250, 176, 5, 255),
        Color(253, 126, 20, 255),
        Color(250, 82, 82, 255),
    ];
    view()
        .class("sapid-demo-marquee-logos")
        .style(
            Style::new()
                .display(glory::style::Display::InlineFlex)
                .flex_direction(if vertical {
                    glory::style::FlexDirection::Column
                } else {
                    glory::style::FlexDirection::Row
                })
                .gap(if wide { 24. } else { 16. }),
        )
        .fill(
            colors
                .into_iter()
                .map(|color| {
                    view()
                        .class("sapid-demo-marquee-logo")
                        .style(Style {
                            align: Some(Align::Center),
                            ..Style::new()
                                .display(glory::style::Display::InlineFlex)
                                .gap(6.)
                        })
                        .fill((
                            view()
                                .style(Style {
                                    background: Some(color),
                                    ..Style::new()
                                        .width(Length::Px(size))
                                        .height(Length::Px(size))
                                        .radius(size / 2.)
                                        .padding(Edges::all(3.))
                                })
                                .fill(
                                    glory::ui::image()
                                        .class("sapid-demo-marquee-symbol")
                                        .alt("")
                                        .source(format!(
                                            "{}/sapid-logo.svg",
                                            option_env!("SAPID_SHOWCASE_MEDIA")
                                                .unwrap_or("/assets")
                                        ))
                                        .style(
                                            Style::new()
                                                .width(Length::Percent(100.))
                                                .height(Length::Percent(100.)),
                                        ),
                                ),
                            text().content("Sapid").style(Style {
                                font_size: Some(if wide { 20. } else { 15. }),
                                ..Style::new().font_weight(700.)
                            }),
                        ))
                })
                .collect::<Vec<_>>(),
        )
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
