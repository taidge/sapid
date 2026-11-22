use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::button()
        .theme(theme)
        .label("Disabled with styles")
        .enabled(false)
        .class("site-outlined-disabled")
}
