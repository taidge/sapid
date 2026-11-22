use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::container()
        .theme(theme)
        .class("site-ref-custom-container")
        .fill(text().content("Container with custom size"))
}
