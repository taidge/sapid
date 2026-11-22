use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(10.).fill(
        [
            (sapid::Size::ExtraSmall, 16.),
            (sapid::Size::Small, 18.),
            (sapid::Size::Medium, 20.),
            (sapid::Size::Large, 24.),
            (sapid::Size::ExtraLarge, 30.),
        ]
        .into_iter()
        .map(|(size, side)| {
            sapid::badge()
                .theme(theme.clone())
                .size(size)
                .variant(sapid::BadgeVariant::Solid)
                .content("1")
                .style(
                    Style {
                        justify: Some(Justify::Center),
                        ..Style::new()
                    }
                    .width(Length::Px(side))
                    .height(Length::Px(side))
                    .radius(side / 2.)
                    .padding(Edges::all(0.)),
                )
        })
        .collect::<Vec<_>>(),
    )
}
