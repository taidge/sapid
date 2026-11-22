use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let arrow_position = scope.cage("center".to_owned());
        let arrow_size = scope.cage(7.);
        let arrow_radius = scope.cage(0.);
        let arrow_offset = scope.cage(10.);
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                arrow_position.get(),
                arrow_size.get() as u16,
                arrow_radius.get() as u16,
                arrow_offset.get() as u16,
            )]
        }))
        .key(Clone::clone)
        .children(move |e| {
            let (pos, size, radius, inset) = e.value.get();
            let mut placement = placement("bottom-start").gap(8.);
            placement.flip = false;
            placement.shift = false;
            placement.arrow = Some(glory::render::AnchorArrow {
                position: if pos == "side" {
                    glory::render::AnchorArrowPosition::Side
                } else {
                    glory::render::AnchorArrowPosition::Center
                },
                size: f64::from(size),
                radius: f64::from(radius).min(f64::from(size) / 2.),
                offset: f64::from(inset),
            });
            sapid::popover("configured-popover")
                .theme(palette.clone())
                .open(true)
                .preserve_focus()
                .show_close(false)
                .dismiss_on_escape(false)
                .dismiss_on_outside(false)
                .width(sapid::PopoverWidth::Px(200.))
                .class("site-position-preview-popover")
                .label("Target element")
                .name("Target element")
                .content(|| {
                    text().content("Arrow position can be changed for *-start and *-end positions")
                })
                .placement(placement)
        });
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::segmented_choice(
                        "popover-arrow-position",
                        "Arrow position",
                        arrow_position,
                        &[("center", "Center"), ("side", "Side")],
                        &theme,
                    ),
                    controls::range(
                        "popover-arrow-offset",
                        "Arrow offset",
                        arrow_offset,
                        5.,
                        50.,
                        1.,
                        &theme,
                    ),
                    controls::range(
                        "popover-arrow-size",
                        "Arrow size",
                        arrow_size,
                        5.,
                        12.,
                        1.,
                        &theme,
                    ),
                    controls::range(
                        "popover-arrow-radius",
                        "Arrow radius",
                        arrow_radius,
                        0.,
                        10.,
                        1.,
                        &theme,
                    ),
                ],
            ));
        panel.show_in(scope)
    })
}

fn placement(value: &str) -> sapid::AnchorPlacement {
    let (side, align) = value.split_once('-').unwrap_or((value, "center"));
    sapid::AnchorPlacement::new(
        match side {
            "top" => sapid::AnchorSide::Top,
            "left" => sapid::AnchorSide::Left,
            "right" => sapid::AnchorSide::Right,
            _ => sapid::AnchorSide::Bottom,
        },
        match align {
            "start" => sapid::AnchorAlign::Start,
            "end" => sapid::AnchorAlign::End,
            _ => sapid::AnchorAlign::Center,
        },
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
