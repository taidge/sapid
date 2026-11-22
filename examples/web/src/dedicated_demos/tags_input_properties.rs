use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "tags-input-props",
        "Input label",
        "Input description",
        move |field, style| {
            sapid::tags_input("tags-input-props-control")
                .field(field)
                .style(style)
                .placeholder("Choose a value")
                .default_value(vec!["React".to_owned()])
        },
    )
}
