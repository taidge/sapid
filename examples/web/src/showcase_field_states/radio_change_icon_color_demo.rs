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
            sapid::radio_group("light-custom-icon-color")
                .theme(theme.clone())
                .name("Custom icon color")
                .class("choice-radio-example")
                .radio(
                    sapid::radio("one")
                        .label("Custom icon color")
                        .class("site-choice-input choice-example choice-lime choice-dark-ink"),
                )
                .default_value("one"),
        )
}
