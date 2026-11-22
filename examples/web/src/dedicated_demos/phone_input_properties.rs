use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "phone-props",
        "Input label",
        "Input description",
        move |field, style| {
            sapid::phone_input("phone-props-control")
                .field(field)
                .editor_style(style)
                .regions([("US", "United States"), ("GB", "United Kingdom")])
                .placeholder("Phone number")
        },
    )
}
