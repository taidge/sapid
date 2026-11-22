use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let position = scope.cage(Some("bottom".to_owned()));
        let main = scope.cage(0.);
        let cross = scope.cage(0.);
        let palette = theme.clone();
        let preview =
            each(scope.bond(move || vec![(position.get(), main.get() as i16, cross.get() as i16)]))
                .key(Clone::clone)
                .children(move |row| {
                    let (position, main, cross) = row.value.get();
                    let mut placement = placement(position.as_deref().unwrap_or("bottom"))
                        .gap(f64::from(main.max(0)));
                    placement.flip = false;
                    placement.shift = false;
                    let overlap = f64::from(main.min(0));
                    let cross = f64::from(cross);
                    let (left, top) = match placement.side {
                        sapid::AnchorSide::Top => (cross, -overlap),
                        sapid::AnchorSide::Bottom => (cross, overlap),
                        sapid::AnchorSide::Left => (-overlap, cross),
                        sapid::AnchorSide::Right => (overlap, cross),
                    };
                    sapid::popover("axes-popover")
                        .theme(palette.clone())
                        .open(true)
                    .preserve_focus()
                    .show_close(false)
                    .dismiss_on_escape(false)
                    .dismiss_on_outside(false)
                    .width(sapid::PopoverWidth::Px(200.))
                    .class("site-position-preview-popover")
                    .label("Popover target")
                    .name("Popover target")
                    .content(|| text().content("Change position and offset to configure dropdown offset relative to target"))
                        .placement(placement)
                        .style(Style::new().margin(glory::style::Edges {
                            left,
                            top,
                            right: 0.,
                            bottom: 0.,
                        }))
                });
        view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::select(
                        "popover-axes-position",
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
                    controls::range(
                        "popover-main-axis",
                        "Main axis",
                        main,
                        -50.,
                        50.,
                        1.,
                        &theme,
                    ),
                    controls::range(
                        "popover-cross-axis",
                        "Cross axis",
                        cross,
                        -50.,
                        50.,
                        1.,
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
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
