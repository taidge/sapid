use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "attachments",
        "Input label",
        "Input description",
        move |field, style| {
            sapid::file_input()
                .field(field)
                .style(style)
                .label("Input placeholder")
                .configuration(glory::render::FileConfig {
                    multiple: true,
                    max_files: 4,
                    ..Default::default()
                })
                .clearable(true)
                .show_attachments(false)
        },
    )
}
