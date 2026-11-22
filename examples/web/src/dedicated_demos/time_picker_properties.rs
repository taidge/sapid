use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "time-picker-properties",
        "Input label",
        "Input description",
        move |field, style| {
            sapid::time_picker("example-time-picker")
                .field(field)
                .style(style)
        },
    )
}
