use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::center()
        .theme(theme)
        .class("site-ref-center-frame")
        .height(Length::Px(100.))
        .fill(
            text()
                .class("site-ref-center-content")
                .content("All elements inside Center are centered"),
        )
}
