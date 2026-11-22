use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example choice-demos-centered")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            sapid::checkbox()
                .theme(theme.clone())
                .label("Bright lime checkbox")
                .class("site-choice-input choice-example choice-lime choice-dark-ink")
                .default_checked(true),
        )
}
