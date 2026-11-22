use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::carousel()
        .theme(theme)
        .name("Images carousel")
        .count(4usize)
        .looping(true)
        .with_indicators(true)
        .render_slide(move |index| {
            let media = option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets");
            let file = ["alpine-lake-photo.png", "team-member-photo.png"][index % 2];
            sapid::image(format!("{media}/{file}"))
                .alt(format!("Gallery photograph {}", index + 1))
                .width(Length::Percent(100.))
                .height(Length::Px(200.))
                .fit(glory::style::ObjectFit::Cover)
        })
}
