use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        view()
            .class("site-reference-actions")
            .on(glory::render::EventKind::Keyboard, move |event| {
                if matches!(
                    event,
                    glory::render::Event::Keyboard {
                        command: glory::render::KeyboardCommand::Dismiss,
                        ..
                    }
                ) {
                    open.revise(|v| *v = false)
                }
            })
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Toggle action bar")
                    .on_press(move || open.revise(|v| *v = !*v)),
                sapid::action_bar("escape-reference")
                    .theme(theme.clone())
                    .visible(open)
                    .selected_count(1usize)
                    .class("site-reference-escape-bar")
                    .name("Press Escape to close this bar")
                    .group(
                        sapid::toolbar_group("reference-actions").button(
                            sapid::button()
                                .theme(theme.clone())
                                .label("Close")
                                .on_press(move || open.revise(|v| *v = false)),
                        ),
                    ),
            ))
            .show_in(scope)
    })
}
