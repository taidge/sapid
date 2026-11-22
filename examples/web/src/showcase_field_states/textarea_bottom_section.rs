use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let message = scope.cage(String::new());
        sapid::field("textarea-message-count")
            .class("site-field-state-example")
            .theme(theme.clone())
            .label("Your message")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(340.)),
            )
            .control(move |field| {
                sapid::textarea()
                    .field(field)
                    .autosize(true)
                    .min_rows(4)
                    .style(Style {
                        padding: Some(Edges {
                            top: 5.5,
                            bottom: 33.5,
                            left: 12.,
                            right: 12.,
                        }),
                        ..Style::new()
                    })
                    .placeholder("Type your message...")
                    .value(message)
                    .on_value_change(move |next| {
                        message.revise(|v| *v = next.chars().take(500).collect())
                    })
                    .bottom_section(
                        text().class("site-field-character-count").content(
                            message.map(|s| format!("{}/500 characters", s.chars().count())),
                        ),
                    )
            })
            .show_in(scope)
    })
}
