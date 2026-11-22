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
            sapid::field("no-negatives-field")
                .theme(theme.clone())
                .label("Negative numbers are not allowed")
                .control(move |field| {
                    sapid::number_input()
                        .placeholder("Do not enter negative numbers")
                        .min(sapid::Decimal::parse("0").ok())
                        .clamp_behavior(sapid::NumberClampBehavior::Strict)
                        .field(field)
                }),
        )
}
