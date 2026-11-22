use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::rich_text_editor()
        .theme(theme)
        .name("Editor with placeholder")
        .placeholder("Start writing here…")
}
