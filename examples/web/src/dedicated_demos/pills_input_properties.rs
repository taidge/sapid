use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "pills-input-props",
        "Input label",
        "Input description",
        move |field, style| {
            sapid::pills_input("pills-input-props-control")
                .field(field)
                .style(style)
                .placeholder("Choose a value")
                .default_value(vec!["React".to_owned()])
        },
    )
}
