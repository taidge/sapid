use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "date-input-properties",
        "Input label",
        "Input description",
        move |field, style| {
            let provider = sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap());
            sapid::date_input("example-date-input", provider)
                .field(field)
                .style(style)
        },
    )
}
