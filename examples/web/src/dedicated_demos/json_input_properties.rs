use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "json-props",
        "Input label",
        "Input description",
        move |field, style| {
            let mut field = Some(field);
            widget(move |scope| {
                let metrics = controls::multiline_playground_style(scope, style, true);
                sapid::json_input("json-props-control")
                    .field(field.take().expect("mount once"))
                    .style(metrics)
                    .placeholder("Input placeholder")
                    .show_formatter(false)
                    .show_in(scope)
            })
        },
    )
}
