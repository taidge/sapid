use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let field = |key: &'static str, label: &'static str, message: &'static str| {
        let color = theme.success;
        sapid::field(key)
            .theme(theme.clone())
            .label(label)
            .success(true)
            .class("site-valid-field")
            .description(message)
            .order([
                sapid::FieldSlot::Label,
                sapid::FieldSlot::Control,
                sapid::FieldSlot::Description,
                sapid::FieldSlot::Error,
            ])
            .control(move |field| {
                sapid::text_field()
                    .field(field)
                    .placeholder(label)
                    .style(Style::new().border_color(color))
            })
    };
    sapid::stack()
        .gap(16.)
        .class("site-field-state-example")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill((
            field("input-success-boolean", "Success as boolean", ""),
            field(
                "input-success-message",
                "Success as react node",
                "Username is available",
            ),
        ))
}
