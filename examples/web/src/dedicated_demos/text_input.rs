use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "text-name",
        "Input label",
        "Input description",
        move |field, style| {
            sapid::text_field()
                .field(field)
                .style(style)
                .placeholder("Input placeholder")
        },
    )
}
