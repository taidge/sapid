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
            sapid::radio_group("radio-check-icon")
                .theme(theme.clone())
                .name("Custom check icon")
                .class("choice-radio-example")
                .radio(
                    sapid::radio("one")
                        .label("Custom check icon")
                        .class("site-choice-input choice-example choice-radio-tick"),
                )
                .default_value("one"),
        )
}
