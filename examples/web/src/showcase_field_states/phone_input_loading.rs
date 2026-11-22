use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .class("site-field-state-example")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            sapid::phone_input("phone-loading")
                .theme(theme)
                .label("Phone number")
                .placeholder("Your phone number")
                .regions([("US", "United States"), ("GB", "United Kingdom")])
                .default_value(sapid::PhoneDraft::new("US", ""))
                .loading(true),
        )
}
