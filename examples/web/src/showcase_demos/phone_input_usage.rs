use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let draft = scope.cage(sapid::PhoneDraft::new("GB", ""));
        sapid::stack()
            .theme(theme.clone())
            .class("site-field-example")
            .gap(8.)
            .fill((
                sapid::phone_input("contact-number")
                    .theme(theme.clone())
                    .label("Contact phone")
                    .country_label("Dialing region")
                    .regions([
                        ("GB", "United Kingdom"),
                        ("US", "United States"),
                        ("CA", "Canada"),
                        ("CN", "China"),
                    ])
                    .placeholder("020 7946 0018")
                    .form_name("phone")
                    .value(draft)
                    .on_value_change(move |next| draft.revise(|value| *value = next)),
                text().content(scope.bond(move || {
                    draft.with(|value| {
                        value.rfc3966().unwrap_or_else(|| {
                            "Enter a complete number to see the form value.".to_owned()
                        })
                    })
                })),
            ))
            .show_in(scope)
    })
}
