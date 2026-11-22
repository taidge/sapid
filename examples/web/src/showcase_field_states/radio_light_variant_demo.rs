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
            sapid::stack().gap(7.).fill((
                sapid::radio_group("light-light-radio")
                    .theme(theme.clone())
                    .name("Light radio")
                    .class("choice-radio-example")
                    .radio(
                        sapid::radio("one")
                            .label("Light radio")
                            .class("site-choice-input choice-example choice-light"),
                    )
                    .default_value("one"),
                sapid::radio_group("light-teal-light-radio")
                    .theme(theme.clone())
                    .name("Teal light radio")
                    .class("choice-radio-example")
                    .radio(
                        sapid::radio("one")
                            .label("Teal light radio")
                            .class("site-choice-input choice-example choice-light choice-teal"),
                    )
                    .default_value("one"),
                sapid::radio_group("light-grape-light-radio")
                    .theme(theme.clone())
                    .name("Grape light radio")
                    .class("choice-radio-example")
                    .radio(
                        sapid::radio("one")
                            .label("Grape light radio")
                            .class("site-choice-input choice-example choice-light choice-grape"),
                    )
                    .default_value("one"),
                sapid::radio_group("light-light-radio-with-css-color")
                    .theme(theme.clone())
                    .name("Light radio with CSS color")
                    .class("choice-radio-example")
                    .radio(
                        sapid::radio("one")
                            .label("Light radio with CSS color")
                            .class("site-choice-input choice-example choice-light choice-pink"),
                    )
                    .default_value("one"),
            )),
        )
}
