use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "autocomplete-props",
        "Input label",
        "Input description",
        move |field, style| {
            sapid::autocomplete("autocomplete-props-control")
                .field(field)
                .style(style)
                .placeholder("Pick a framework")
                .options([
                    sapid::autocomplete_option("react", "React"),
                    sapid::autocomplete_option("angular", "Angular"),
                    sapid::autocomplete_option("vue", "Vue"),
                ])
        },
    )
}
