use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::color_picker("styles-picker")
        .theme(theme)
        .name("Color picker")
        .size(sapid::Size::Large)
        .default_value(sapid::Hsva::from_rgba(glory::style::Color(
            255, 255, 255, 255,
        )))
        .show_alpha(true)
        .plane_size(280.)
        .plane_height(162.)
        .swatches_per_row(7)
        .swatches([
            "#25262b", "#868e96", "#fa5252", "#e64980", "#be4bdb", "#7950f2", "#4c6ef5",
        ]);
    crate::showcase_style_explorer::explorer(
        "color-picker-styles",
        Box::new(
            sapid::stack()
                .class("site-field-state-example")
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .max_width(Limit::Px(280.)),
                )
                .fill(preview),
        ),
        &[
            ("wrapper", "Root element", ".sapid-color-picker"),
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
        ],
    )
}
