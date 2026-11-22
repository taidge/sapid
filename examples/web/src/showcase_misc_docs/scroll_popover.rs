use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::popover("scroll-results-popover")
        .theme(theme.clone())
        .label("Browse results")
        .width(sapid::PopoverWidth::Px(290.))
        .content(move || {
            let palette = theme.clone();
            sapid::scroll_area("popup-result-list")
                .theme(theme.clone())
                .name("Result list")
                .style(
                    Style::new()
                        .height(Length::Auto)
                        .max_height(Limit::Px(170.)),
                )
                .content(move |_| {
                    sapid::stack().theme(palette.clone()).gap(8.).fill(
                        (1..=24)
                            .map(|i| {
                                sapid::text()
                                    .theme(palette.clone())
                                    .content(format!("Result {i}"))
                            })
                            .collect::<Vec<_>>(),
                    )
                })
        })
}
