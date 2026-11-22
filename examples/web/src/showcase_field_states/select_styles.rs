use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::field("select-styles-field")
        .theme(theme.clone())
        .label("Select")
        .description("Description")
        .required(true)
        .control(move |field| {
            sapid::select("styles-select")
                .placeholder("Select")
                .open(true)
                .group(
                    sapid::select_group("Frontend")
                        .options(["React", "Angular"].map(|v| sapid::select_option(v, v))),
                )
                .group(
                    sapid::select_group("Backend")
                        .options(["Node", "Django"].map(|v| sapid::select_option(v, v))),
                )
                .field(field)
        });
    crate::showcase_style_explorer::explorer(
        "select-styles",
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
                ".sapid-select-shell",
            ),
            ("input", "Input element", "[role=\"combobox\"]"),
            (
                "section",
                "Left and right sections",
                ".sapid-select-chevron",
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
            ("dropdown", "Dropdown root element", "[role=\"listbox\"]"),
            ("options", "Options wrapper", "[role=\"listbox\"]"),
            ("option", "Option", "[role=\"option\"]"),
            ("empty", "Nothing found message", ".sapid-select-empty"),
            ("group", "Options group wrapper", "[role=\"group\"]"),
            (
                "groupLabel",
                "Options group label",
                "[role=\"group\"] > [data-gly-kind=\"text\"]",
            ),
        ],
    )
}
