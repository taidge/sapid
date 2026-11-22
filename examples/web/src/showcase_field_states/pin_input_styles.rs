use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::pin_input()
        .theme(theme)
        .name("PIN")
        .length(4)
        .gap(12.)
        .placeholder("○");
    crate::showcase_style_explorer::explorer(
        "pin-input-styles",
        Box::new(
            sapid::stack()
                .class("site-field-state-example")
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .max_width(Limit::Px(340.)),
                )
                .fill(preview),
        ),
        &[
            ("root", "Root element", ".sapid-pin-input"),
            ("pinInput", "Input item wrapper", ".sapid-pin-frame"),
            ("input", "Input element", ".sapid-pin-cell"),
        ],
    )
}
