use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::field("radio-styles-field")
        .theme(theme.clone())
        .label("Radio")
        .description("Radio description")
        .error("Radio error")
        .invalid(true)
        .control(move |field| {
            sapid::radio_group("styles-radio")
                .default_value("radio")
                .items([sapid::radio("radio").label("Radio")])
                .field(field)
        });
    crate::showcase_style_explorer::explorer(
        "radio-styles",
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
            ("root", "Root element", ".sapid-field"),
            (
                "radio",
                "Input element (`input[type=\"radio\"]`)",
                "input[type=\"radio\"]",
            ),
            (
                "icon",
                "Radio icon, used to display checked icon",
                "input:checked",
            ),
            ("inner", "Wrapper for `icon` and `input`", "input"),
            (
                "body",
                "Input body, contains all other elements",
                ".sapid-field",
            ),
            (
                "labelWrapper",
                "Contains `label`, `description` and `error`",
                ".sapid-field-label",
            ),
            ("label", "Label element", ".sapid-field-label"),
            (
                "description",
                "Description displayed below the label",
                ".sapid-field-description",
            ),
            (
                "error",
                "Error message displayed below the label",
                ".sapid-field-error",
            ),
        ],
    )
}
