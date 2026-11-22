use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "accent-field",
        "Input label",
        "Input description",
        move |field, style| {
            sapid::color_input("accent-color")
                .field(field)
                .style(style)
                .placeholder("Input placeholder")
                .default_value("")
                .format(sapid::ColorFormat::Hexa)
                .swatches(["#228be6b3", "#12b886b3", "#fa5252b3"])
        },
    )
}
