use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::field("checkbox-styles-field")
        .theme(theme.clone())
        .label("Checkbox")
        .description("Checkbox description")
        .error("Checkbox error")
        .invalid(true)
        .control(move |field| sapid::checkbox().default_checked(true).field(field));
    crate::showcase_style_explorer::explorer(
        "checkbox-styles",
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
                "input",
                "Input element (`input[type=\"checkbox\"]`)",
                "input[type=\"checkbox\"]",
            ),
            (
                "icon",
                "Checkbox icon, used to display checkmark and indeterminate state icon",
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
