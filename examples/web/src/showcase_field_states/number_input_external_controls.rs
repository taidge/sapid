use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let decimal = |n| sapid::Decimal::new(n, 0).expect("bounded integer");
        let value = scope.cage(sapid::NumberValue::from(decimal(15)));
        let change = move |step: i128| {
            let number = match value.get().parse(sapid::NumberLocale::EnUs) {
                sapid::NumberParse::Complete(number) => number.to_f64(),
                _ => 10.,
            };
            let next = (number + step as f64).clamp(10., 20.);
            if let Ok(next) = sapid::Decimal::parse(&next.to_string()) {
                value.revise(|v| *v = next.into());
            }
        };
        sapid::stack()
            .gap(16.)
            .class("site-field-state-example site-external-number-actions")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(340.)),
            )
            .fill((
                sapid::field("external-number-field")
                    .label("Click buttons to change value")
                    .control(move |field| {
                        sapid::number_input()
                            .field(field)
                            .placeholder("Click the buttons")
                            .min(Some(decimal(10)))
                            .max(Some(decimal(20)))
                            .step(decimal(2))
                            .value(value)
                            .on_value_change(move |next| value.revise(|v| *v = next))
                    }),
                sapid::group().gap(16.).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Decrement by 2")
                        .on_press(move || change(-2)),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Increment by 2")
                        .on_press(move || change(2)),
                )),
            ))
            .show_in(scope)
    })
}
