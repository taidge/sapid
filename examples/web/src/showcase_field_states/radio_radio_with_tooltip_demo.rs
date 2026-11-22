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
        .fill(sapid::stack().gap(7.).fill((
            {
                let theme = theme.clone();
                sapid::tooltip("radio-input-tip")
                    .theme(theme.clone())
                    .text("Radio with tooltip")
                    .target(move |target| {
                        target.view(
                            view()
                                .class("choice-tooltip-target choice-tooltip-input")
                                .fill(
                                    sapid::radio_group("light-tooltip-on-radio-only")
                                        .theme(theme.clone())
                                        .name("Tooltip on radio only")
                                        .class("choice-radio-example")
                                        .radio(
                                            sapid::radio("one")
                                                .label("Tooltip on radio only")
                                                .class("site-choice-input choice-example "),
                                        ),
                                ),
                        )
                    })
            },
            {
                let theme = theme.clone();
                sapid::tooltip("radio-label-tip")
                    .theme(theme.clone())
                    .text("Radio with tooltip")
                    .target(move |target| {
                        target.view(
                            view().class("choice-tooltip-target ").fill(
                                sapid::radio_group("light-tooltip-the-entire-element")
                                    .theme(theme.clone())
                                    .name("Tooltip the entire element")
                                    .class("choice-radio-example")
                                    .radio(
                                        sapid::radio("one")
                                            .label("Tooltip the entire element")
                                            .class("site-choice-input choice-example "),
                                    ),
                            ),
                        )
                    })
            },
        )))
}
