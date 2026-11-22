use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "date-time-picker-properties",
        "Input label",
        "Input description",
        move |field, style| {
            let provider = sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap());
            sapid::date_time_picker("example-date-time", provider, sapid::DateTimePolicy::Civil)
                .field(field)
                .style(style)
        },
    )
}
