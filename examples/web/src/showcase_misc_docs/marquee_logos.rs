pub(crate) fn logo_strip(vertical: bool, wide: bool) -> impl glory::ui::Widget {
    use glory::prelude::*;
    let size = if wide { 32. } else { 22. };
    let colors = [
        Color(34, 139, 230, 255),
        Color(21, 170, 191, 255),
        Color(18, 184, 134, 255),
        Color(64, 192, 87, 255),
        Color(130, 201, 30, 255),
        Color(250, 176, 5, 255),
        Color(253, 126, 20, 255),
        Color(250, 82, 82, 255),
    ];
    view()
        .class("sapid-demo-marquee-logos")
        .style(
            Style::new()
                .display(glory::style::Display::InlineFlex)
                .flex_direction(if vertical {
                    glory::style::FlexDirection::Column
                } else {
                    glory::style::FlexDirection::Row
                })
                .gap(if wide { 24. } else { 16. }),
        )
        .fill(
            colors
                .into_iter()
                .map(|color| {
                    view()
                        .class("sapid-demo-marquee-logo")
                        .style(Style {
                            align: Some(Align::Center),
                            ..Style::new()
                                .display(glory::style::Display::InlineFlex)
                                .gap(6.)
                        })
                        .fill((
                            view()
                                .style(Style {
                                    background: Some(color),
                                    ..Style::new()
                                        .width(Length::Px(size))
                                        .height(Length::Px(size))
                                        .radius(size / 2.)
                                        .padding(Edges::all(3.))
                                })
                                .fill(
                                    glory::ui::image()
                                        .class("sapid-demo-marquee-symbol")
                                        .alt("")
                                        .source(format!(
                                            "{}/sapid-logo.svg",
                                            option_env!("SAPID_SHOWCASE_MEDIA")
                                                .unwrap_or("/assets")
                                        ))
                                        .style(
                                            Style::new()
                                                .width(Length::Percent(100.))
                                                .height(Length::Percent(100.)),
                                        ),
                                ),
                            text().content("Sapid").style(Style {
                                font_size: Some(if wide { 20. } else { 15. }),
                                ..Style::new().font_weight(700.)
                            }),
                        ))
                })
                .collect::<Vec<_>>(),
        )
}
