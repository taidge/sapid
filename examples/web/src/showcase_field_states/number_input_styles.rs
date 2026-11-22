use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::field("number-input-styles-field")
        .theme(theme.clone())
        .label("Label")
        .description("Description")
        .required(true)
        .error("Error")
        .invalid(true)
        .control(move |field| {
            sapid::number_input()
                .placeholder("NumberInput")
                .field(field)
        });
    crate::showcase_style_explorer::explorer(
        "number-input-styles",
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
            (
                "wrapper",
                "Root element of the Input",
                ".sapid-number-input-row",
            ),
            ("input", "Input element", ".sapid-number-input"),
            (
                "section",
                "Left and right sections",
                ".sapid-number-input-controls",
            ),
            (
                "bottomSection",
                "Bottom section element, rendered inside the input border at the bottom",
                "",
            ),
            ("root", "Root element", ".sapid-field"),
            ("label", "Label element", ".sapid-field-label"),
            (
                "required",
                "Required asterisk element, rendered inside label",
                ".sapid-field-label::after",
            ),
            (
                "description",
                "Description element",
                ".sapid-field-description",
            ),
            ("error", "Error element", ".sapid-field-error"),
            (
                "success",
                "Success element",
                ".site-valid-field .sapid-field-description",
            ),
            (
                "controls",
                "Increment and decrement buttons wrapper",
                ".sapid-number-input-controls",
            ),
            (
                "control",
                "Increment and decrement buttons",
                ".sapid-number-input-control",
            ),
        ],
    )
}
