use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let accordion = sapid::accordion("fruit-unstyled")
        .theme(theme.clone())
        .unstyled();
    display_sample::fruit_preview(theme, accordion, None, 600.)
}
