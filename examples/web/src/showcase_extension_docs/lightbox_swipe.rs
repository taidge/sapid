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
        let enabled = scope.cage(true);
        sapid::group()
            .theme(theme.clone())
            .fill((
                sapid::lightbox("swipe-gallery")
                    .theme(theme.clone())
                    .label("Open lightbox")
                    .images(images)
                    .close_on_swipe(enabled),
                sapid::switch()
                    .theme(theme.clone())
                    .label("Swipe to close")
                    .checked(enabled)
                    .on_checked_change(move |next| enabled.revise(|v| *v = next)),
            ))
            .show_in(scope)
    })
}
