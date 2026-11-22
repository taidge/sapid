use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::picker_field_playground(
        theme,
        "month-picker-input-properties",
        "Input label",
        "Input description",
        Some("Pick month"),
        true,
        move |field, style, placeholder| {
            let provider = sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap());
            sapid::month_picker_input("example-month-picker-input", provider)
                .field(field)
                .style(style)
                .placeholder(placeholder)
        },
    )
}
