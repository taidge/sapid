use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::rich_text_editor()
        .theme(theme)
        .name("Subtle editor")
        .subtle(true)
        .placeholder("Write a note…")
}
