use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("phone_input_success-accepted")
        .theme(theme.clone())
        .label("Phone number")
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
            sapid::phone_input("phone_input_success-accepted")
                .regions([("US", "United States"), ("GB", "United Kingdom")])
                .field(field)
                .editor_style(Style::new().border_color(theme.success))
                .placeholder("Phone number")
        })
}
