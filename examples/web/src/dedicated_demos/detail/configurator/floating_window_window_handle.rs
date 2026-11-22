use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let bounds = scope.cage(sapid::WindowBounds {
            x: 40.,
            y: 200.,
            width: 280.,
            height: 160.,
        });
        let handle = scope.cage(None::<sapid::FloatingWindowHandle>);
        let controls = sapid::group()
            .theme(theme.clone())
            .class("site-reference-actions")
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .variant(sapid::ButtonVariant::Secondary)
                    .label(open.map(|v| {
                        if *v {
                            "Hide floating window".to_owned()
                        } else {
                            "Show floating window".to_owned()
                        }
                    }))
                    .on_press(move || open.revise(|v| *v = !*v)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Set position")
                    .on_press(move || {
                        if let Some(h) = handle.get() {
                            let _ = h.set_bounds(sapid::WindowBounds {
                                x: 100.,
                                y: 100.,
                                ..bounds.get()
                            });
                        }
                    }),
            ));
        let window = sapid::floating_window("ref-window-handle")
            .theme(theme.clone())
            .title("use-floating-window hook")
            .bounds(bounds)
            .open(open)
            .on_open_change(move |v| open.revise(|s| *s = v))
            .on_bounds_change(move |next| {
                bounds.revise(|v| *v = next);
            })
            .on_ready(move |h| handle.revise(|s| *s = Some(h)))
            .content(|| text().content("The position control moves this window."));
        group().fill((controls, window)).show_in(scope)
    })
}
