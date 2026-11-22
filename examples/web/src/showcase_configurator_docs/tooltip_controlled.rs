use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        sapid::tooltip("reference-controlled")
            .theme(theme.clone())
            .open(open)
            .text("Ctrl + J")
            .target(move |target| {
                target.button(
                    glory::ui::button()
                        .label("Toggle color scheme")
                        .class("site-reference-target")
                        .on(glory::render::EventKind::Activate, move |_| {
                            open.revise(|v| *v = !*v)
                        }),
                )
            })
            .show_in(scope)
    })
}
