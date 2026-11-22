use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let side = scope.cage(Some("right".to_owned()));
        let offset = scope.cage(true);
        let preview = {
            let theme = theme.clone();

            sapid::scroll_area("reading-scrollbar-side")
                .name("Reading with a positioned scrollbar")
                .theme(theme.clone())
                .scrollbars(sapid::Scrollbars::Always)
                .vertical_scrollbar_position(side.map(|v| {
                    if v.as_deref() == Some("left") {
                        sapid::VerticalScrollbarPosition::Left
                    } else {
                        sapid::VerticalScrollbarPosition::Right
                    }
                }))
                .offset_scrollbars(offset.map(|v| {
                    if *v {
                        sapid::ScrollbarOffset::Both
                    } else {
                        sapid::ScrollbarOffset::None
                    }
                }))
                .style(
                    Style::new()
                        .width(Length::Px(300.))
                        .max_width(Limit::Percent(100.))
                        .height(Length::Px(200.)),
                )
                .content(move |_| display_sample::reading_article(theme.clone(), false))
        };
        view()
            .class("site-configurator site-display-configurator site-scroll-side-play-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::option_segments(
                        "scroll_side_play-side",
                        "Vertical scrollbar position",
                        side,
                        &[("left", "Left"), ("right", "Right")],
                        &theme,
                    ),
                    controls::switch("Offset scrollbars", offset, &theme),
                ],
            ))
            .show_in(scope)
    })
}
