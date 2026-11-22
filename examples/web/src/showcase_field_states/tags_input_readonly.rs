use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("tags_input_readonly-example")
        .theme(theme.clone())
        .class("site-field-state-example")
        .label("Read only")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .readonly(true)
        .control(move |field| {
            sapid::tags_input("tags_input_readonly-example")
                .field(field)
                .placeholder("Read only")
                .default_value(vec!["React".to_owned()])
        })
}
