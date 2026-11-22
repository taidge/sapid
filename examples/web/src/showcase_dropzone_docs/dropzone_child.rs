use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let clicks = scope.cage(0usize);
        view()
            .class("showcase-dropzone-child")
            .fill((
                sapid::dropzone()
                    .theme(theme.clone())
                    .label("Drop files or choose files")
                    .style(Style::new().min_height(Length::Px(160.))),
                sapid::button()
                    .theme(theme.clone())
                    .class("showcase-dropzone-action")
                    .label(clicks.map(|n| format!("Show file requirements ({n})")))
                    .on_press(move || clicks.revise(|n| *n += 1)),
                when(clicks.map(|n| *n > 0), || {
                    text().content("Choose a file that fits the configured size limit.")
                }),
            ))
            .show_in(scope)
    })
}
