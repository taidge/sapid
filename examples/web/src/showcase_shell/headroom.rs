use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let show = scope.cage(true);
        let last = scope.cage(0.);
        sapid::app_shell("headroom-shell")
            .theme(theme.clone())
            .style(Style::new().height(Length::Px(290.)))
            .header_visible(show)
            .header(|c| sapid::text().theme(c.theme).content("Workspace header"))
            .main(move |c| {
                let palette = c.theme;
                sapid::scroll_area("headroom-article")
                    .theme(palette.clone())
                    .name("Scroll to hide or reveal the header")
                    .style(Style::new().height(Length::Px(210.)))
                    .on_scroll_change(move |(_, y)| {
                        let previous = last.get();
                        if (previous - y).abs() >= 1. {
                            show.revise(|v| *v = y < previous || y < 16.);
                            last.revise(|v| *v = y);
                        }
                    })
                    .content(move |_| {
                        sapid::stack().theme(palette.clone()).gap(16.).fill(
                            (1..=25)
                                .map(|n| {
                                    sapid::text()
                                        .theme(palette.clone())
                                        .content(format!("Article paragraph {n}"))
                                })
                                .collect::<Vec<_>>(),
                        )
                    })
            })
            .show_in(scope)
    })
}
