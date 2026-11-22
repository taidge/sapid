use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example choice-demos-left")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            view()
                .class("choice-indicator-row")
                .semantics(Some(Box::new(glory::render::Semantics {
                    inert: true,
                    ..Default::default()
                })))
                .fill(
                    sapid::group().gap(16.).fill((
                        sapid::radio_group("radio-indicator-0")
                            .theme(theme.clone())
                            .name("Radio indicator")
                            .class("choice-radio-example")
                            .radio(
                                sapid::radio("one")
                                    .name("Selection indicator")
                                    .class("site-choice-input choice-example "),
                            ),
                        sapid::radio_group("radio-indicator-1")
                            .theme(theme.clone())
                            .name("Radio indicator")
                            .class("choice-radio-example")
                            .radio(
                                sapid::radio("one")
                                    .name("Selection indicator")
                                    .class("site-choice-input choice-example "),
                            )
                            .default_value("one"),
                        sapid::radio_group("radio-indicator-2")
                            .theme(theme.clone())
                            .name("Radio indicator")
                            .class("choice-radio-example")
                            .radio(
                                sapid::radio("one")
                                    .name("Selection indicator")
                                    .class("site-choice-input choice-example ")
                                    .enabled(false),
                            ),
                        sapid::radio_group("radio-indicator-3")
                            .theme(theme.clone())
                            .name("Radio indicator")
                            .class("choice-radio-example")
                            .radio(
                                sapid::radio("one")
                                    .name("Selection indicator")
                                    .class("site-choice-input choice-example ")
                                    .enabled(false),
                            )
                            .default_value("one"),
                    )),
                ),
        )
}
