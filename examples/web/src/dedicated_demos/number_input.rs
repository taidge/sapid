use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "number-quantity",
        "Input label",
        "Input description",
        move |field, style| {
            sapid::number_input()
                .field(field)
                .style(style)
                .placeholder("Input placeholder")
        },
    )
}
