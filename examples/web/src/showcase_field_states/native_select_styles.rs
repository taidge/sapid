use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::field("native-select-styles-field")
        .theme(theme.clone())
        .label("NativeSelect label")
        .description("NativeSelect description")
        .required(true)
        .error("NativeSelect error")
        .invalid(true)
        .control(move |field| {
            sapid::native_select("styles-native")
                .options(["React", "Angular"].map(|v| sapid::select_option(v, v)))
                .default_value(Some("React".into()))
                .right_section(
                    glory::ui::graphic()
                        .drawing(sapid::Icon::Selector.drawing(theme.muted))
                        .style(Style::new().width(Length::Px(18.)).height(Length::Px(18.))),
                )
                .field(field)
        });
    crate::showcase_style_explorer::explorer(
        "native-select-styles",
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
                "wrapper",
                "Root element of the Input",
                ".sapid-native-select-frame",
            ),
            ("input", "Input element", ".sapid-native-select"),
            (
                "section",
                "Left and right sections",
                ".sapid-native-select-right",
            ),
            (
                "bottomSection",
                "Bottom section element, rendered inside the input border at the bottom",
                "",
            ),
        ],
    )
}
