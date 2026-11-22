use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("json_input_success-accepted")
        .theme(theme.clone())
        .label("JSON config")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .success(true)
        .class("site-field-state-example site-valid-field")
        .description("Valid JSON")
        .order([
            sapid::FieldSlot::Label,
            sapid::FieldSlot::Control,
            sapid::FieldSlot::Description,
            sapid::FieldSlot::Error,
        ])
        .control(move |field| {
            sapid::json_input("json_input_success-accepted")
                .field(field)
                .style(Style::new().border_color(theme.success))
                .placeholder("JSON config")
                .show_formatter(false)
        })
}
