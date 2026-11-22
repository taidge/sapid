use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let panels = [false, true].map(|delayed| {
            let layer = scope.cage(false);
            let visible = scope.cage(false);
            let data = scope.cage(false);
            let body = theme.clone();
            sapid::dialog(format!("modal-exit-{delayed}"))
                .theme(theme.clone())
                .open(layer)
                .title("Edit your profile")
                .label(if delayed {
                    "Clear data in onExitTransitionEnd"
                } else {
                    "Clear data in onClose"
                })
                .on_open_change(move |next| {
                    if next {
                        data.revise(|v| *v = true);
                        layer.revise(|v| *v = true);
                        visible.revise(|v| *v = true);
                    } else {
                        if !delayed {
                            data.revise(|v| *v = false);
                        }
                        visible.revise(|v| *v = false);
                    }
                })
                .content(move || {
                    let body = body.clone();
                    sapid::transition()
                        .mounted(visible)
                        .appear(true)
                        .duration(300)
                        .exit_duration(1000)
                        .on_exited(move || {
                            data.revise(|v| *v = false);
                            layer.revise(|v| *v = false);
                        })
                        .content(move || {
                            sapid::stack().theme(body.clone()).gap(12.).fill((
                                when(data, || text().content("Imagine a form here")),
                                sapid::button().theme(body.clone()).label("Close").on_press(
                                    move || {
                                        if !delayed {
                                            data.revise(|v| *v = false);
                                        }
                                        visible.revise(|v| *v = false);
                                    },
                                ),
                            ))
                        })
                })
        });
        sapid::group()
            .theme(theme.clone())
            .class("site-reference-actions")
            .fill(panels.into_iter().collect::<Vec<_>>())
            .show_in(scope)
    })
}
