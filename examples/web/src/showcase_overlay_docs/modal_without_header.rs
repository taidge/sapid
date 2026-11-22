use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let body = theme.clone();
        sapid::dialog("modal-no-header")
            .theme(theme.clone())
            .label("Open without header")
            .name("Panel without header")
            .show_close(false)
            .open(open)
            .on_open_change(move |next| open.revise(|v| *v = next))
            .content(move || {
                sapid::stack().theme(body.clone()).gap(12.).fill((
                    text().content("The application supplies the closing action."),
                    sapid::button()
                        .theme(body.clone())
                        .label("Done")
                        .on_press(move || open.revise(|v| *v = false)),
                ))
            })
            .show_in(scope)
    })
}
