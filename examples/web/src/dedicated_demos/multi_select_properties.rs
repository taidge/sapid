use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "multi-select-props",
        "Input label",
        "Input description",
        move |field, style| {
            sapid::multi_select("multi-select-props-control")
                .field(field)
                .style(style)
                .placeholder("Choose a value")
                .options([
                    sapid::select_option("react", "React"),
                    sapid::select_option("angular", "Angular"),
                    sapid::select_option("vue", "Vue"),
                ])
        },
    )
}
