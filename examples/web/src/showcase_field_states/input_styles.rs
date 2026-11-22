use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::text_field()
        .theme(theme)
        .name("Input component")
        .placeholder("Input component")
        .left_section(text().content("@"))
        .right_section(text().content("⌄"));
    crate::showcase_style_explorer::explorer(
        "input-styles",
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
            ("wrapper", "Root element of the Input", ".sapid-text-field"),
            ("input", "Input element", "input"),
            (
                "section",
                "Left and right sections",
                ".sapid-input-left-section,.sapid-input-right-section",
            ),
            (
                "bottomSection",
                "Bottom section element, rendered inside the input border at the bottom",
                ".sapid-input-bottom-section",
            ),
        ],
    )
}
