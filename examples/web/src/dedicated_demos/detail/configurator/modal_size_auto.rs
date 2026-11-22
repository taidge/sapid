use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let count = scope.cage(3usize);
        let body = theme.clone();
        let badges = scope.bond(move || (0..count.get()).collect::<Vec<_>>());
        sapid::dialog("reference-auto-modal")
            .theme(theme.clone())
            .label("Open modal")
            .title("Modal size auto")
            .class("site-reference-modal-auto")
            .content(move || {
                sapid::stack().theme(body.clone()).gap(16.).fill((
                    text().content("Modal with size auto fits its content"),
                    sapid::group()
                        .theme(body.clone())
                        .class("site-reference-modal-badges")
                        .fill(each(badges).key(Clone::clone).children(|row| {
                            text()
                                .class("site-reference-badge")
                                .content(format!("Badge {}", row.value.get()))
                        })),
                    sapid::group().theme(body.clone()).fill((
                        sapid::button()
                            .theme(body.clone())
                            .label("Add badge")
                            .on_press(move || count.revise(|n| *n = n.saturating_add(1))),
                        sapid::button()
                            .theme(body.clone())
                            .label("Remove badge")
                            .on_press(move || count.revise(|n| *n = n.saturating_sub(1))),
                    )),
                ))
            })
            .show_in(scope)
    })
}
