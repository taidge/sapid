use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let accordion = sapid::accordion("fruit-transition-none")
        .theme(theme.clone())
        .transition_duration(0);
    display_sample::fruit_preview(theme, accordion, None, 380.)
}
