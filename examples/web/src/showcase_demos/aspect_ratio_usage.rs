use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::aspect_ratio(1080. / 720.)
        .theme(theme.clone())
        .class("site-ref-ratio-image")
        .fill(
            sapid::image(format!(
                "{}/reference-ratio-panda.png",
                option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
            ))
            .theme(theme)
            .alt("Panda")
            .width(Length::Percent(100.))
            .height(Length::Percent(100.))
            .fit(glory::style::ObjectFit::Cover),
        )
}
