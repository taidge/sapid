use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(false);
        let hovering = scope.cage(false);
        let count = scope.cage(0usize);
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::{JsCast, closure::Closure};
            let document = web_sys::window()
                .and_then(|win| win.document())
                .expect("browser document");
            let depth = std::rc::Rc::new(std::cell::Cell::new(0u32));
            for name in ["dragenter", "dragleave", "dragend"] {
                let depth = depth.clone();
                let callback =
                    Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                        if !active.get() {
                            depth.set(0);
                            return;
                        }
                        if name == "dragenter" {
                            let has_files = event
                                .dyn_ref::<web_sys::DragEvent>()
                                .and_then(web_sys::DragEvent::data_transfer)
                                .is_some_and(|transfer| {
                                    transfer
                                        .types()
                                        .iter()
                                        .any(|kind| kind.as_string().as_deref() == Some("Files"))
                                });
                            if !has_files {
                                return;
                            }
                            depth.set(depth.get().saturating_add(1));
                            hovering.revise(|v| *v = true);
                        } else {
                            depth.set(if name == "dragend" {
                                0
                            } else {
                                depth.get().saturating_sub(1)
                            });
                            if depth.get() == 0 {
                                hovering.revise(|v| *v = false);
                            }
                        }
                    });
                document
                    .add_event_listener_with_callback(name, callback.as_ref().unchecked_ref())
                    .expect("drag listener");
                let target = document.clone();
                scope.on_cleanup(move || {
                    let _ = target.remove_event_listener_with_callback(
                        name,
                        callback.as_ref().unchecked_ref(),
                    );
                })?;
            }
        }
        let palette = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .class("showcase-fullscreen-sample")
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label(active.map(|on| {
                        if *on {
                            "Deactivate full screen dropzone".to_owned()
                        } else {
                            "Activate full screen dropzone".to_owned()
                        }
                    }))
                    .on_press(move || {
                        active.revise(|on| *on = !*on);
                        hovering.revise(|v| *v = false);
                    }),
                when(
                    scope.bond(move || active.get() && hovering.get()),
                    move || {
                        view().class("showcase-fullscreen-drop").fill(
                            super::image_dropzone::image_dropzone(palette.clone()).on_files(
                                move |files| {
                                    count.revise(|v| *v = files.len());
                                    active.revise(|v| *v = false);
                                    hovering.revise(|v| *v = false);
                                },
                            ),
                        )
                    },
                ),
                when(count.map(|n| *n > 0), move || {
                    text().content(count.map(|value| format!("{value} files accepted")))
                }),
            ))
            .show_in(scope)
    })
}
