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
        let controls = sapid::group()
            .theme(theme.clone())
            .class("site-reference-actions")
            .fill(
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
            );
        let window = sapid::floating_window("ref-constrain-offset")
            .theme(theme.clone())
            .title("Constrain offset")
            .bounds(bounds)
            .open(open)
            .on_open_change(move |v| open.revise(|s| *s = v))
            .on_bounds_change(move |next| {
                #[cfg(target_arch = "wasm32")]
                let next = {
                    let mut next = next;
                    if let Some(win) = web_sys::window() {
                        let w = win
                            .inner_width()
                            .ok()
                            .and_then(|v| v.as_f64())
                            .unwrap_or(1024.);
                        let h = win
                            .inner_height()
                            .ok()
                            .and_then(|v| v.as_f64())
                            .unwrap_or(768.);
                        next.x = next.x.clamp(30., (w - next.width - 30.).max(30.));
                        next.y = next.y.clamp(30., (h - next.height - 30.).max(30.));
                    }
                    next
                };
                bounds.revise(|v| *v = next);
            })
            .content(|| text().content("Drag this window within a 30px viewport margin."));
        group().fill((controls, window)).show_in(scope)
    })
}
