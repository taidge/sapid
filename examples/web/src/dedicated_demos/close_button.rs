use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(true);
        let card_theme = theme.clone();
        let restore_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .fill((
                when(open, move || {
                    sapid::group().theme(card_theme.clone()).gap(12.).fill((
                        sapid::text()
                            .theme(card_theme.clone())
                            .content("The offline trail map is ready."),
                        sapid::close_button()
                            .theme(card_theme.clone())
                            .name("Close notification")
                            .on_press(move || open.revise(|v| *v = false)),
                    ))
                }),
                when(open.map(|v| !*v), move || {
                    sapid::button()
                        .theme(restore_theme.clone())
                        .label("Restore notification")
                        .on_press(move || open.revise(|v| *v = true))
                }),
            ))
            .show_in(scope)
    })
}
