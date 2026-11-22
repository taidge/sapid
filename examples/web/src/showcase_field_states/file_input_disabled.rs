use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("file_input_disabled-example")
        .theme(theme.clone())
        .class("site-field-state-example")
        .label("Disabled input")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .enabled(false)
        .control(move |field| sapid::file_input().field(field).label("Disabled input"))
}
