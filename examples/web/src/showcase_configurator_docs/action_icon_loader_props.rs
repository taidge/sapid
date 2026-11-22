use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::action_icon("Loading...")
        .theme(theme.clone())
        .size(sapid::Size::ExtraLarge)
        .loading(true)
        .loader_type(sapid::LoaderType::Dots)
        .class("site-reference-loading-icon")
        .icon(glory::ui::graphic().drawing(sapid::Icon::Check.drawing(theme.foreground)))
}
