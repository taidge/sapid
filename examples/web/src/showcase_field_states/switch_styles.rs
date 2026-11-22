use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::field("switch-styles-field")
        .theme(theme.clone())
        .label("Switch component")
        .size(sapid::Size::Small)
        .description("Switch description")
        .error("Switch error")
        .invalid(true)
        .control(move |field| {
            sapid::switch()
                .default_checked(true)
                .with_thumb_indicator(true)
                .with_marks(true)
                .field(field)
        });
    crate::showcase_style_explorer::explorer(
        "switch-styles",
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
                "track",
                "Switch track, contains `thumb` and `trackLabel`",
                ".sapid-switch-track",
            ),
            (
                "trackLabel",
                "Label displayed inside `track`",
                ".sapid-switch-mark",
            ),
            (
                "thumb",
                "Thumb displayed inside `track`",
                ".sapid-switch-thumb",
            ),
            (
                "input",
                "Input element (`input[type=\"checkbox\"]`), hidden by default",
                "input[type=\"checkbox\"]",
            ),
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
