use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let visible = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .class("site-reference-inline-loading")
            .fill((
                sapid::loading_overlay(visible)
                    .theme(theme.clone())
                    .class("site-reference-inline-loader")
                    .label("Loading...")
                    .loader_content(text().content("Loading..."))
                    .fill(controls::authentication(theme.clone(), false)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Toggle overlay")
                    .on_press(move || visible.revise(|v| *v = !*v)),
            ))
            .show_in(scope)
    })
}
