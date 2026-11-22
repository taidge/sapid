use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .fill(
            view()
                .style(
                    Style::new()
                        .width(Length::Px(500.))
                        .max_width(Limit::Percent(100.)),
                )
                .fill(
                    sapid::scroller("badge-scroll-step")
                        .scroll_step(100.)
                        .area(|area| {
                            area.theme(theme.clone())
                                .name("Badges with fixed scroll distance")
                                .style(Style::new().height(Length::Px(46.)))
                        })
                        .content(move |_| badges(theme.clone())),
                ),
        )
}

fn badges(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .wrap(glory::style::FlexWrap::NoWrap)
        .gap(8.)
        .fill(
            (1..=24)
                .map(|n| {
                    sapid::badge()
                        .theme(theme.clone())
                        .content(format!("Badge {n}"))
                        .style(Style::new().min_width(Length::Px(72.)))
                })
                .collect::<Vec<_>>(),
        )
}
