use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::field("color-input-styles-field")
        .theme(theme.clone())
        .label("Label")
        .description("Description")
        .required(true)
        .error("Error")
        .invalid(true)
        .control(move |field| {
            sapid::color_input("styles-color")
                .placeholder("ColorInput")
                .format(sapid::ColorFormat::Rgba)
                .eye_dropper(crate::showcase_host::eye_dropper())
                .swatches(["#000000", "#ffffff", "#ff0000", "#00ff00", "#0000ff"])
                .field(field)
        });
    crate::showcase_style_explorer::explorer(
        "color-input-styles",
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
            ("wrapper", "Root element", ".sapid-color-input"),
            ("input", "Input element", "input"),
            (
                "section",
                "Left and right sections",
                ".sapid-color-input-preview,.sapid-color-input-eye-dropper",
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
                "preview",
                "Color preview, displayed only when `format` supports alpha channel",
                ".sapid-color-picker-preview",
            ),
            (
                "body",
                "Contains alpha/hue sliders and color preview",
                ".sapid-color-picker-body",
            ),
            (
                "slider",
                "Alpha and hue sliders root",
                ".sapid-hue-slider,.sapid-alpha-slider",
            ),
            (
                "sliderOverlay",
                "Element used to display various overlays over hue and alpha sliders",
                ".sapid-color-slider-overlay",
            ),
            ("saturation", "Saturation picker", ".sapid-color-plane"),
            (
                "saturationOverlay",
                "Element used to display various overlays over saturation picker",
                ".sapid-color-plane-saturation,.sapid-color-plane-value",
            ),
            (
                "sliders",
                "Contains alpha and hue sliders",
                ".sapid-color-picker-sliders",
            ),
            (
                "thumb",
                "Thumb of all sliders",
                ".sapid-slider-thumb,.sapid-color-plane-indicator",
            ),
            ("swatch", "Color swatch", ".sapid-color-picker-swatch"),
            (
                "swatches",
                "Color swatches list",
                ".sapid-color-picker-swatches",
            ),
            ("dropdown", "Popover dropdown", ".sapid-color-picker"),
            (
                "colorPreview",
                "Color swatch preview in input left section",
                ".sapid-color-input-preview",
            ),
            (
                "eyeDropperButton",
                "Eye dropper button",
                ".sapid-color-input-eye-dropper",
            ),
            (
                "eyeDropperIcon",
                "Default eye dropper icon",
                ".sapid-color-input-eye-dropper svg",
            ),
        ],
    )
}
