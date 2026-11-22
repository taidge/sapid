use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .fill(
            sapid::scroll_area("offset-reading")
                .theme(theme.clone())
                .name("Charizard reading area")
                .scrollbars(sapid::Scrollbars::Always)
                .scrollbar_size(6.)
                .offset_scrollbars(sapid::ScrollbarOffset::Vertical)
                .class("site-offset-reading")
                .style(
                    Style::new()
                        .width(Length::Px(300.))
                        .max_width(Limit::Percent(100.))
                        .height(Length::Px(200.)),
                )
                .content({
                    let article_theme = theme.clone();
                    move |_| display_sample::reading_article(article_theme.clone(), false)
                }),
        )
}
