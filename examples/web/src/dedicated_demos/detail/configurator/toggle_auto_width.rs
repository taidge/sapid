use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .class("site-reference-actions")
        .fill((
            sapid::toggle()
                .theme(theme.clone())
                .label("Save")
                .class("site-auto-width-toggle"),
            sapid::toggle()
                .theme(theme.clone())
                .label("Active")
                .class("site-auto-width-toggle")
                .default_pressed(true),
            sapid::toggle()
                .theme(theme)
                .label("Disabled")
                .class("site-auto-width-toggle")
                .enabled(false),
        ))
}
