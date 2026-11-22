use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::drawer("inset-drawer")
        .theme(theme)
        .class("site-inset-drawer")
        .label("Open inset drawer")
        .title("Authentication")
        .description("Sign in to continue.")
}
