use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let media = option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets");
    let images = [
        ("lake", "alpine-lake-photo.png", "Alpine lake"),
        ("portrait", "team-member-photo.png", "Studio portrait"),
    ]
    .map(|(key, file, name)| {
        sapid::lightbox_image(key, format!("{media}/{file}"), name).caption(name)
    })
    .to_vec();
    sapid::lightbox("lightbox-loop")
        .theme(theme)
        .images(images)
        .label("Open gallery")
        .title("Loop navigation")
        .loop_navigation(true)
}
