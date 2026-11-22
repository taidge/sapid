use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let host = sapid::aspect_ratio(16. / 9.)
            .theme(theme.clone())
            .class("site-ratio-video");
        host.show_in(scope)?;
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::{JsCast, closure::Closure};
            if let Some(win) = web_sys::window()
                && let Some(doc) = win.document()
            {
                let target = doc.clone();
                let mount = Closure::once(move || {
                    if let Ok(Some(el)) = target.query_selector(".site-ratio-video")
                        && el.query_selector("iframe").ok().flatten().is_none()
                        && let Ok(frame) = target.create_element("iframe")
                    {
                        let _ =
                            frame.set_attribute("src", "https://www.youtube.com/embed/mzJ4vCjSt28");
                        let _ = frame.set_attribute("title", "YouTube video player");
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
