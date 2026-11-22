use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let media = option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets");
        let images = [
            ("lake", "alpine-lake-photo.png", "Alpine lake"),
            ("ridge", "mountain-evening.svg", "Mountain evening"),
        ]
        .map(|(key, file, caption)| {
            sapid::lightbox_image(key, format!("{media}/{file}"), caption).caption(caption)
        })
        .to_vec();
        let details = scope.cage(false);
        let view = scope.cage(sapid::LightboxView::default());
        let palette = theme.clone();
        sapid::lightbox("toolbar-gallery")
            .theme(theme.clone())
            .label("Open lightbox with custom toolbar")
            .images(images)
            .view(view)
            .on_view_change(move |value| view.revise(|old| *old = value))
            .toolbar(move || {
                let palette = palette.clone();
                sapid::group().theme(palette.clone()).fill((
                    sapid::button()
                        .theme(palette)
                        .label("Image info")
                        .on_press(move || details.revise(|v| *v = !*v)),
                    when(details, move || {
                        text().content(view.map(|v| {
                            if v.current.as_deref() == Some("ridge") {
                                "Mountain evening · Illustration".to_owned()
                            } else {
                                "Alpine lake · Photograph".to_owned()
                            }
                        }))
                    }),
                ))
            })
            .show_in(scope)
    })
}
