use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let position = scope.cage(Some("bottom".to_owned()));
        let offset = scope.cage(5.);
        let arrow = scope.cage(false);
        let arrow_position = scope.cage(Some("side".to_owned()));
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                position.get(),
                offset.get() as i16,
                arrow.get(),
                arrow_position.get(),
            )]
        }))
        .key(Clone::clone)
        .children(move |e| {
            let (position, offset, arrow, pos) = e.value.get();
            let mut placement =
                placement(position.as_deref().unwrap_or("bottom")).gap(f64::from(offset.max(0)));
            placement.arrow = arrow.then_some(glory::render::AnchorArrow {
                position: if pos.as_deref() == Some("side") {
                    glory::render::AnchorArrowPosition::Side
                } else {
                    glory::render::AnchorArrowPosition::Center
                },
                size: 8.,
                radius: 1.,
                offset: 10.,
            });
            let overlap = f64::from(offset.min(0));
            let (left, top) = match placement.side {
                sapid::AnchorSide::Top => (0., -overlap),
                sapid::AnchorSide::Bottom => (0., overlap),
                sapid::AnchorSide::Left => (-overlap, 0.),
                sapid::AnchorSide::Right => (overlap, 0.),
            };
            sapid::dropdown_menu("configured-menu")
                .theme(palette.clone())
                .label("Toggle menu")
                .position(placement)
                .style(Style::new().margin(glory::style::Edges {
                    left,
                    top,
                    right: 0.,
                    bottom: 0.,
                }))
                .item(sapid::menu_item("settings").label("Settings"))
                .item(sapid::menu_item("search").label("Search"))
        });
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::select(
                        "menu-position",
                        "Position",
                        position,
                        &[
                            ("top", "top"),
                            ("top-start", "top-start"),
                            ("top-end", "top-end"),
                            ("bottom", "bottom"),
                            ("bottom-start", "bottom-start"),
                            ("bottom-end", "bottom-end"),
                            ("left", "left"),
                            ("left-start", "left-start"),
                            ("left-end", "left-end"),
                            ("right", "right"),
                            ("right-start", "right-start"),
                            ("right-end", "right-end"),
                        ],
                        &theme,
                    ),
                    controls::range("menu-offset", "Offset", offset, -20., 20., 1., &theme),
                    controls::switch("With arrow", arrow, &theme),
                    controls::select(
                        "menu-arrow-position",
                        "Arrow position",
                        arrow_position,
                        &[("side", "Side"), ("center", "Center")],
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

    pub fn select(
        key: &str,
        label: &str,
        value: Cage<Option<String>>,
        options: &'static [(&'static str, &'static str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let field_key = format!("menu-dropdown-position-{key}");
        let select_key = format!("menu-dropdown-position-select-{key}");
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
