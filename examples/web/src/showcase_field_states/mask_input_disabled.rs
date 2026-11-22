use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("mask_input_disabled-example")
        .theme(theme.clone())
        .class("site-field-state-example")
        .label("Phone number")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .enabled(false)
        .control(move |field| {
            sapid::mask_input(sapid::MaskPattern::parse("(###) ###-####").unwrap())
                .field(field)
                .placeholder("Phone number")
        })
}
