use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::button()
        .theme(theme)
        .label("Loading button")
        .loading(true)
        .loader_type(sapid::LoaderType::Dots)
        .class("site-reference-loading-button")
}
