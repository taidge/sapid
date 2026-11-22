use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let media = option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets");
    let images = [
        ("lake", "alpine-lake-photo.png", "Alpine lake"),
        ("ridge", "mountain-evening.svg", "Mountain evening"),
    ]
    .map(|(key, file, caption)| {
        sapid::lightbox_image(key, format!("{media}/{file}"), caption).caption(caption)
    })
    .to_vec();
    let palette = theme.clone();
    sapid::lightbox("video-gallery")
        .theme(theme)
        .label("Open lightbox with videos")
        .images(images)
        .render_slide(move |slide| -> Box<dyn Widget> {
            if slide.key == "lake" {
                Box::new(widget(move |scope| {
                    #[cfg(target_arch = "wasm32")]
                    {
                        use wasm_bindgen::JsCast;
                        let video = std::rc::Rc::new(std::cell::RefCell::new(
                            None::<web_sys::HtmlVideoElement>,
                        ));
                        let after = video.clone();
                        scope.on_cleanup(move || {
                            if let Some(v) = after.borrow_mut().take() {
                                let _ = v.pause();
                                v.remove_attribute("src").ok();
                                v.load();
                                v.remove();
                            }
                        })?;
                        scope.spawn(async move {
                            super::notifications_system_timeouts::delay(0).await;
                            let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
                                return;
                            };
                            let Some(host) = doc.get_element_by_id("showcase-lightbox-video-host")
                            else {
                                return;
                            };
                            let Ok(element) = doc.create_element("video") else {
                                return;
                            };
                            let Ok(player) = element.dyn_into::<web_sys::HtmlVideoElement>() else {
                                return;
                            };
                            player.set_controls(true);
                            player.set_muted(true);
                            player.set_autoplay(true);
                            player.set_loop(true);
                            let _ = player.set_attribute("playsinline", "");
                            let _ = player.set_attribute("aria-label", "Alpine lake film");
                            player.set_src(&format!("{media}/alpine-lake-motion.webm"));
                            if host.append_child(&player).is_ok() {
                                *video.borrow_mut() = Some(player);
                            }
                        })?;
                    }
                    view()
                        .class("showcase-lightbox-video")
                        .semantics(Some(Box::new(glory::render::Semantics {
                            id: Some(glory::render::SemanticId::new(
                                "showcase-lightbox-video-host",
                            )?),
                            ..Default::default()
                        })))
                        .show_in(scope)
                }))
            } else {
                Box::new(
                    sapid::image(slide.source)
                        .theme(palette.clone())
                        .alt(slide.alternative)
                        .fit(glory::style::ObjectFit::Contain)
                        .style(
                            Style::new()
                                .width(Length::Percent(100.))
                                .height(Length::Px(320.)),
                        ),
                )
            }
        })
}
