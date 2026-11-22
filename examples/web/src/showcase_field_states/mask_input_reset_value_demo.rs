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
            let value = scope.cage("4155550123".to_owned());
            sapid::stack()
                .theme(theme.clone())
                .gap(12.)
                .fill((
                    sapid::mask_input(sapid::MaskPattern::parse("(###) ###-####").unwrap())
                        .theme(theme.clone())
                        .name("Phone number")
                        .value(value)
                        .on_value_change(move |next| value.revise(|v| *v = next)),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Reset value")
                        .on_press(move || value.revise(String::clear)),
                ))
                .show_in(scope)
        }))
}
