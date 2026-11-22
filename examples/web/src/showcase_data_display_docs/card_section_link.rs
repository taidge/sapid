use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::card()
        .theme(theme.clone())
        .bordered(true)
        .title("Norway fjord adventures")
        .header(
            sapid::anchor()
                .theme(theme.clone())
                .name("Explore the destination")
                .href("#card-sections")
                .fill(
                    sapid::image(format!(
                        "{}/alpine-lake-photo.png",
                        option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
                    ))
                    .alt("A lakeside village")
                    .style(
                        Style::new()
                            .height(Length::Px(130.))
                            .width(Length::Percent(100.)),
                    ),
                ),
        )
        .fill(
            sapid::text()
                .theme(theme)
                .content("Discover lakeside villages and walking routes."),
        )
}
