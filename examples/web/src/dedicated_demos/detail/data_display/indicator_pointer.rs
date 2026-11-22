use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).fill(
        sapid::indicator()
            .theme(theme.clone())
            .diameter(16.)
            .with_border(false)
            .label("New")
            .class("site-inline-indicator")
            .fill(display_sample::portrait(theme.clone(), true, false)),
    )
}
