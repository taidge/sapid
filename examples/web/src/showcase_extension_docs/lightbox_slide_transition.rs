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
    sapid::lightbox("slide-motion-gallery")
        .theme(theme)
        .label("Open lightbox with slide transition")
        .images(images)
        .loop_navigation(true)
        .render_slide(move |slide| {
            let palette = palette.clone();
            sapid::transition()
                .theme(palette.clone())
                .class("showcase-lightbox-slide-motion")
                .mounted(true)
                .appear(true)
                .effect(sapid::TransitionEffect::SlideLeft)
                .duration(260)
                .content(move || {
                    sapid::image(slide.source.clone())
                        .alt(slide.alternative.clone())
                        .theme(palette.clone())
                        .fit(glory::style::ObjectFit::Contain)
                        .style(
                            Style::new()
                                .width(Length::Percent(100.))
                                .height(Length::Px(320.)),
                        )
                })
        })
}
