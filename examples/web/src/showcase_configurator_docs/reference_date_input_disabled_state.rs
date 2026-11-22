use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("demo-date-input-1-field")
        .theme(theme.clone())
        .label("Date input")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .control(move |field| {
            sapid::date_input(
                "demo-date-input-1",
                DatesProvider::new(CivilDate::parse_iso("2025-04-15").unwrap()),
            )
            .field(field)
            .enabled(false)
            .default_value(None)
        })
}
