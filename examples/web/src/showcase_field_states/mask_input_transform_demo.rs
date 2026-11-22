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
        .fill(widget(move |scope| {
            let value = scope.cage(String::new());
            sapid::mask_input(sapid::MaskPattern::parse("AAA-###").unwrap())
                .theme(theme.clone())
                .name("Uppercase code")
                .placeholder("ABC-123")
                .value(value)
                .on_value_change(move |next| value.revise(|v| *v = next.to_ascii_uppercase()))
                .show_in(scope)
        }))
}
