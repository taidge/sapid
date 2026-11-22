use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let point = scope.cage((0., 0.));
        let ink = theme.clone();
        let current = scope.cage(String::new());
        let target = view()
            .class("site-context-menu-area")
            .role(glory::render::Role::Group)
            .name("Context menu area")
            .focus(glory::render::FocusMode::TabStop)
            .semantics(Some(Box::new(glory::render::Semantics {
                id: Some(glory::render::SemanticId::new("reference-cursor-profile")?),
                context_menu: Some(Default::default()),
                ..Default::default()
            })))
            .on(glory::render::EventKind::ContextMenu, move |event| {
                if let glory::render::Event::ContextMenu { point: at, .. } = event {
                    point.revise(|v| *v = (at.x, at.y));
                    open.revise(|v| *v = true);
                }
            })
            .fill((
                text().content("Right-click anywhere inside this area"),
                text().content("A popover will open at the cursor position"),
            ));
        let pop = sapid::popover("profile-context")
            .theme(theme.clone())
            .open(open)
            .on_open_change(move |v| open.revise(|s| *s = v))
            .class("site-ref-context-popover")
            .label("Profile actions")
            .title("Jane Doe")
            .close_label("Close profile")
            .width(sapid::PopoverWidth::Px(260.))
            .trigger_style(point.map(|(x, y)| {
                Style::new()
                    .position(glory::style::Position::Fixed)
                    .left(Length::Px(*x))
                    .top(Length::Px(*y))
                    .width(Length::Px(1.))
                    .height(Length::Px(1.))
            }))
            .content(move || {
                sapid::stack().theme(ink.clone()).gap(10.).fill((
                    text().content("Jane Doe"),
                    text().content("jane@example.com"),
                    sapid::group().theme(ink.clone()).gap(8.).fill(
                        ["Message", "Follow"]
                            .map(|name| {
                                sapid::button()
                                    .theme(ink.clone())
                                    .label(name)
                                    .on_press(move || {
                                        current.revise(|s| *s = name.to_owned());
                                        open.revise(|v| *v = false);
                                    })
                            })
                            .into_iter()
                            .collect::<Vec<_>>(),
                    ),
                ))
            });
        group()
            .fill((target, pop, text().content(current)))
            .show_in(scope)
    })
}
