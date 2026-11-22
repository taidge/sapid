use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .class("site-format-toggles")
        .gap(12.)
        .fill(Vec::from(
            [
                ("B", "Bold", "bold"),
                ("I", "Italic", "italic"),
                ("U", "Underline", "underline"),
            ]
            .map(|(glyph, label, decoration)| {
                sapid::toggle()
                    .theme(theme.clone())
                    .label(glyph)
                    .name(label)
                    .class(format!("site-toggle-{decoration}"))
            }),
        ))
}
