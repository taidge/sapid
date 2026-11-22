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
            sapid::pin_input()
                .theme(theme.clone())
                .name("Four digits between 0 and 3")
                .length(4)
                .placeholder("○")
                .alphabet(sapid::PinAlphabet::DigitRange { first: 0, last: 3 }),
        )
}
