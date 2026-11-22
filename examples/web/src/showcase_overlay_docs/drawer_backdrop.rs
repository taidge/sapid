use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::drawer("drawer-custom-backdrop")
        .theme(theme)
        .class("site-tinted-backdrop")
        .label("Open with custom overlay")
        .title("Custom overlay")
        .description("Click outside the panel or press Escape to close it.")
}
