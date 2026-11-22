use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("native_select_success-accepted")
        .theme(theme.clone())
        .label("Native Select")
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
            sapid::native_select("native_select_success-accepted")
                .options(["React", "Angular", "Svelte", "Vue"].map(|v| sapid::select_option(v, v)))
                .field(field)
                .style(Style::new().border_color(theme.success))
                .placeholder("Native Select")
        })
}
