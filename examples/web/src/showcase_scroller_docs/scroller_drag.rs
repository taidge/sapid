use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::scroller("drag-scroll-badges")
        .mouse_drag(true)
        .area(|area| {
            area.theme(theme.clone()).name("Browse badges").style(
                Style::new()
                    .height(Length::Px(55.))
                    .max_width(Limit::Px(350.)),
            )
        })
        .content(move |_| {
            sapid::group()
                .theme(theme.clone())
                .wrap(glory::style::FlexWrap::NoWrap)
                .gap(10.)
                .fill(
                    (1..=12)
                        .map(|n| {
                            sapid::badge()
                                .theme(theme.clone())
                                .content(format!("Badge {n}"))
                        })
                        .collect::<Vec<_>>(),
                )
        })
}
