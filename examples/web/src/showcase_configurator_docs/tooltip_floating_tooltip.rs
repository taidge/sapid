use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        #[cfg(target_arch = "wasm32")]
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            use wasm_bindgen::{JsCast, closure::Closure};
            let root = doc
                .document_element()
                .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok());
            let movement = Closure::<dyn FnMut(web_sys::PointerEvent)>::new(
                move |ev: web_sys::PointerEvent| {
                    let over = ev
                        .target()
                        .and_then(|v| v.dyn_into::<web_sys::Element>().ok())
                        .and_then(|el| el.closest(".site-ref-pointer-surface").ok().flatten())
                        .is_some();
                    if let Some(el) = &root {
                        if over {
                            let _ = el.style().set_property(
                                "--site-ref-pointer-x",
                                &format!("{}px", ev.client_x() + 12),
                            );
                            let _ = el.style().set_property(
                                "--site-ref-pointer-y",
                                &format!("{}px", ev.client_y() + 12),
                            );
                            let _ = el.set_attribute("data-site-ref-pointer", "");
                        } else {
                            let _ = el.remove_attribute("data-site-ref-pointer");
                        }
                    }
                },
            );
            let _ = doc
                .add_event_listener_with_callback("pointermove", movement.as_ref().unchecked_ref());
            scope.on_cleanup(move || {
                let _ = doc.remove_event_listener_with_callback(
                    "pointermove",
                    movement.as_ref().unchecked_ref(),
                );
                if let Some(el) = doc.document_element() {
                    let _ = el.remove_attribute("data-site-ref-pointer");
                }
            })?;
        }
        view()
            .class("site-ref-tour-centered")
            .fill(
                sapid::tooltip("reference-floating-tip")
                    .theme(theme.clone())
                    .text("Floating tooltip")
                    .class("site-ref-pointer-tip")
                    .target(|target| {
                        target.button(
                            glory::ui::button()
                                .label("Hover over the box to see tooltip")
                                .class("site-ref-pointer-surface"),
                        )
                    }),
            )
            .show_in(scope)
    })
}
