use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("text_input_success-accepted")
        .theme(theme.clone())
        .label("Text Input")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .success(true)
        .class("site-field-state-example site-valid-field")
        .description("Looks good!")
        .order([
            sapid::FieldSlot::Label,
            sapid::FieldSlot::Control,
            sapid::FieldSlot::Description,
            sapid::FieldSlot::Error,
        ])
        .control(move |field| {
            sapid::text_field()
                .field(field)
                .style(Style::new().border_color(theme.success))
                .placeholder("Text Input")
        })
}
