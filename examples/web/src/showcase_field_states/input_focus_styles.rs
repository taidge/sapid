use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .gap(16.)
        .class("site-field-state-example site-custom-input-focus")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill((
            sapid::text_field()
                .theme(theme.clone())
                .name("Regular Input component")
                .placeholder("Regular Input component"),
            sapid::field("custom-focus-field")
                .theme(theme)
                .label("TextInput component")
                .control(|field| {
                    sapid::text_field()
                        .field(field)
                        .placeholder("TextInput component")
                }),
        ))
}
