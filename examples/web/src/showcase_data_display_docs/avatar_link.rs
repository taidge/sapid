use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::anchor()
        .theme(theme.clone())
        .href("#avatar-variant-play")
        .name("See avatar variants")
        .fill(sapid::avatar("Ava Patel").theme(theme).source(format!(
            "{}/team-member-photo.png",
            option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
        )))
}
