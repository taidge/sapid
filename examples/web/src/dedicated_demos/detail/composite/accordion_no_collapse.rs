use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let accordion = sapid::accordion("fruit-no-collapse")
        .theme(theme.clone())
        .collapsible(false)
        .default_open(["apples"]);
    display_sample::fruit_preview(theme, accordion, None, 600.)
}
