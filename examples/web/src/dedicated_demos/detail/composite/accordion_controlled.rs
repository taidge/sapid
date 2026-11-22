use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let accordion = sapid::accordion("fruit-controlled")
        .theme(theme.clone())
        .default_open(["apples"]);
    display_sample::fruit_preview(theme, accordion, Some("bananas"), 600.)
}
