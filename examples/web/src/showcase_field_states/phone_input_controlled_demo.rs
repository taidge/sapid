use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let draft = scope.cage(sapid::PhoneDraft::new("US", "+1"));
        sapid::stack()
            .gap(12.)
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(340.)),
            )
            .fill((
                sapid::phone_input("phone-controlled")
                    .theme(theme.clone())
                    .label("Phone number")
                    .value(draft)
                    .on_value_change(move |next| draft.revise(|value| *value = next)),
                text().content(draft.map(|d| {
                    format!(
                        "{} · {}",
                        d.region,
                        d.rfc3966().unwrap_or_else(|| d.text.clone())
                    )
                })),
            ))
            .show_in(scope)
    })
}
