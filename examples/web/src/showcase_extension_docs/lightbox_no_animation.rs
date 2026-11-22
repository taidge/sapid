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
    sapid::lightbox("instant-gallery")
        .theme(theme)
        .label("Open lightbox without animation")
        .images(images)
        .transition(sapid::TransitionEffect::Pop, 0)
}
