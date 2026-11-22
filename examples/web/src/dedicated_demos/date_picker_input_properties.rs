use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::picker_field_playground(
        theme,
        "date-picker-input-properties",
        "Input label",
        "Input description",
        Some("Pick date"),
        true,
        move |field, style, placeholder| {
            let provider = sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap());
            sapid::date_picker_input("example-date-picker-input", provider)
                .field(field)
                .style(style)
                .placeholder(placeholder)
        },
    )
}
