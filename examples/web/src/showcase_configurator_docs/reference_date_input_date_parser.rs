use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("demo-date-input-2-field")
        .theme(theme.clone())
        .label("Date input")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .control(move |field| {
            sapid::date_input(
                "demo-date-input-2",
                DatesProvider::new(CivilDate::parse_iso("2025-04-15").unwrap()),
            )
            .field(field)
            .placeholder("Type WW2")
            .parser(|input| {
                if input.eq_ignore_ascii_case("WW2") {
                    DateParse::Date(CivilDate::parse_iso("1939-09-01").unwrap())
                } else {
                    DateInputFormat::Iso.parse(input)
                }
            })
            .default_value(None)
        })
}
