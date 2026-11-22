use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "textarea-bio",
        "Input label",
        "Input description",
        move |field, style| {
            let mut field = Some(field);
            widget(move |scope| {
                sapid::textarea()
                    .field(field.take().expect("mount once"))
                    .style(controls::multiline_playground_style(scope, style, false))
                    .rows(2)
                    .placeholder("Input placeholder")
                    .show_in(scope)
            })
        },
    )
}
