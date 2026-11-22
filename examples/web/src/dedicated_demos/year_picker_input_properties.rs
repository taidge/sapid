use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::picker_field_playground(
        theme,
        "year-picker-input-properties",
        "Input label",
        "Input description",
        Some("Pick year"),
        true,
        move |field, style, placeholder| {
            let provider = sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap());
            sapid::year_picker_input("example-year-picker-input", provider)
                .field(field)
                .style(style)
                .placeholder(placeholder)
        },
    )
}
