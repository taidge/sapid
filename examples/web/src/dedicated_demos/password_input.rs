use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "password-usage",
        "Input label",
        "Input description",
        move |field, style| {
            sapid::password_input()
                .field(field)
                .style(style)
                .placeholder("Input placeholder")
        },
    )
}
