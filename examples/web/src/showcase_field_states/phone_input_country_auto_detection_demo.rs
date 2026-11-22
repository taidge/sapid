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
            sapid::phone_input("phone-country-auto-detection")
                .theme(theme.clone())
                .label("Phone number")
                .placeholder("Type +44 to switch to UK")
                .default_value(sapid::PhoneDraft::new("US", ""))
                .regions([
                    ("US", "United States"),
                    ("GB", "United Kingdom"),
                    ("FR", "France"),
                ]),
        )
}
