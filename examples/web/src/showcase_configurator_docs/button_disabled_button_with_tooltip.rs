use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::tooltip("disabled-button-hint")
        .theme(theme)
        .text("Tooltip for disabled button")
        .target(|target| {
            target.button(
                glory::ui::button()
                    .label("Disabled button with tooltip")
                    .class("site-disabled-button-link"),
            )
        })
}
