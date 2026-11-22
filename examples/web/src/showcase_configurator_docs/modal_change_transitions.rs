use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .class("site-reference-actions")
        .fill((
            sapid::dialog("reference-rotating-modal")
                .theme(theme.clone())
                .label("Rotate left transition")
                .title("Please consider this")
                .class("site-reference-rotate-modal")
                .content(|| text().content("rotate-left transition")),
            sapid::dialog("reference-fading-modal")
                .theme(theme)
                .label("Fade transition")
                .title("Please consider this")
                .class("site-reference-fade-modal")
                .content(|| text().content("fade transition 600ms linear transition")),
        ))
}
