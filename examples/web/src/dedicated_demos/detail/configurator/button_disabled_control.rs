use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::center().theme(theme.clone()).fill(
        sapid::button()
            .theme(theme)
            .class("site-ref-single-disabled")
            .enabled(false)
            .label("Disabled button"),
    )
}
