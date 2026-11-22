use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::anchor()
        .theme(theme)
        .label("Anchor component")
        .href("?widget=anchor#usage")
        .open_in(glory::render::LinkOpen::NewContext)
}
