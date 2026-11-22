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
