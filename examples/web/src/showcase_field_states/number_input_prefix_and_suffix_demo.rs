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
        .fill((
            sapid::field("number-prefix")
                .theme(theme.clone())
                .label("With prefix")
                .control(move |field| {
                    sapid::input_group().inset_start(text().content("$")).input(
                        sapid::number_input()
                            .default_value(sapid::Decimal::parse("100").unwrap())
                            .controls(false)
                            .field(field),
                    )
                }),
            sapid::field("number-suffix")
                .theme(theme.clone())
                .label("With suffix")
                .control(move |field| {
                    sapid::input_group().inset_end(text().content("%")).input(
                        sapid::number_input()
                            .default_value(sapid::Decimal::parse("100").unwrap())
                            .controls(false)
                            .field(field),
                    )
                }),
        ))
}
