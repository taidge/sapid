use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::badge()
        .theme(theme)
        .variant(sapid::BadgeVariant::Solid)
        .content("Full width badge")
        .style(
            Style {
                justify: Some(Justify::Center),
                ..Style::new()
            }
            .width(Length::Percent(100.)),
        )
}
