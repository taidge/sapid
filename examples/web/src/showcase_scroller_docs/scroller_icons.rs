use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let left = glory::ui::graphic()
        .drawing(sapid::Icon::ChevronLeft.drawing(theme.accent))
        .style(Style::new().width(Length::Px(14.)).height(Length::Px(14.)));
    let right = glory::ui::graphic()
        .drawing(sapid::Icon::ArrowRight.drawing(theme.accent))
        .style(Style::new().width(Length::Px(14.)).height(Length::Px(14.)));
    sapid::scroller("custom-scroll-icons")
        .previous_icon(left)
        .next_icon(right)
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
