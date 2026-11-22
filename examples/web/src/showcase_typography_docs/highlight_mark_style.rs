use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut palette = theme;
    palette.accent_soft = Color(215, 234, 255, 255);
    palette.accent_text = Color(21, 95, 184, 255);
    view().class("sapid-demo-highlight-emphasis").fill(
        sapid::highlight()
            .theme(palette)
            .content("Change the highlighted text style to emphasize this match.")
            .query("highlighted"),
    )
}
