use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    view().style(Style::new().display(Display::Flex)).fill(
        sapid::aspect_ratio(1.)
            .theme(theme.clone())
            .style(Style::new().width(Length::Px(100.)))
            .fill(
                sapid::image(format!(
                    "{}/reference-ratio-avatar.png",
                    option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
                ))
                .theme(theme)
                .alt("Avatar")
                .width(Length::Percent(100.))
                .height(Length::Percent(100.))
                .fit(glory::style::ObjectFit::Cover),
            ),
    )
}
