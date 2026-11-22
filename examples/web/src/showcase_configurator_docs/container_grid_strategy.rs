use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::container()
        .theme(theme)
        .max_width(Limit::Px(500.))
        .class("site-ref-grid-container")
        .fill((
            view()
                .class("site-ref-grid-content")
                .fill(text().content("Main content")),
            view().class("site-ref-grid-breakout").fill((
                text().content("Breakout"),
                view()
                    .class("site-ref-grid-inner")
                    .fill(text().content("Container inside breakout")),
            )),
        ))
}
