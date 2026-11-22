use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            sapid::field("safe-integer-field")
                .theme(theme.clone())
                .label("Beyond the safe integer limit")
                .control(move |field| {
                    sapid::number_input()
                        .default_value(sapid::Decimal::parse("9007199254740993").unwrap())
                        .field(field)
                }),
        )
}
