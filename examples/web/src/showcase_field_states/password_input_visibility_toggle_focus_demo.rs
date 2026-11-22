use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example choice-demos-field")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            sapid::stack().gap(7.).fill((
                sapid::field("visibility-focus-email-field")
                    .theme(theme.clone())
                    .label("Email")
                    .control(move |field| {
                        sapid::text_field()
                            .placeholder("your@email.com")
                            .field(field)
                    }),
                sapid::field("visibility-focus-password-field")
                    .theme(theme.clone())
                    .label("Password")
                    .control(move |field| {
                        sapid::password_input().default_value("secret").field(field)
                    }),
                sapid::field("visibility-focus-nickname-field")
                    .theme(theme.clone())
                    .label("Nickname")
                    .control(move |field| {
                        sapid::text_field()
                            .placeholder("Your nickname")
                            .field(field)
                    }),
            )),
        )
}
