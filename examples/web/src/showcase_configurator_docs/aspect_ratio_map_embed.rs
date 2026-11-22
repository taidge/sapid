use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let host = sapid::aspect_ratio(16. / 9.)
            .theme(theme.clone())
            .class("site-ratio-map");
        host.show_in(scope)?;
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::{JsCast, closure::Closure};
            if let Some(win) = web_sys::window()
                && let Some(doc) = win.document()
            {
                let target = doc.clone();
                let mount = Closure::once(move || {
                    if let Ok(Some(el)) = target.query_selector(".site-ratio-map")
                        && el.query_selector("iframe").ok().flatten().is_none()
                        && let Ok(frame) = target.create_element("iframe")
                    {
                        let _=frame.set_attribute("src","https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3025.3063874233135!2d-74.04668908358428!3d40.68924937933441!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x89c25090129c363d%3A0x40c6a5770d25022b!2sStatue%20of%20Liberty%20National%20Monument!5e0!3m2!1sen!2sru!4v1644262070010!5m2!1sen!2sru");
                        let _ = frame.set_attribute("title", "Google map");
                        let _ = frame.set_attribute("loading", "lazy");
                        let _ = frame.set_attribute("allowfullscreen", "");
                        let _ = el.append_child(&frame);
                    }
                });
                let timer = win
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        mount.as_ref().unchecked_ref(),
                        0,
                    )
                    .ok();
                scope.on_cleanup(move || {
                    if let Some(id) = timer {
                        win.clear_timeout_with_handle(id);
                    }
                    drop(mount);
                })?;
            }
        }
        Ok(())
    })
}
