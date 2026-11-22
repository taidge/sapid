use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::anchor()
        .theme(theme)
        .href("?widget=button")
        .label("Disabled link")
        .class("site-disabled-button-link")
        .on_navigate(|_| {})
}
