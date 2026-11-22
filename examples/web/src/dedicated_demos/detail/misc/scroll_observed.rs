use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let pos = scope.cage((0., 0.));
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Center)
            .gap(16.)
            .fill((
                sapid::scroll_area("position-subscription")
                    .theme(theme.clone())
                    .name("Charizard reading area")
                    .axes(glory::render::ScrollAxes::Both)
                    .on_scroll_change(move |next| pos.revise(|v| *v = next))
                    .style(
                        Style::new()
                            .width(Length::Px(300.))
                            .max_width(Limit::Percent(100.))
                            .height(Length::Px(200.)),
                    )
                    .content({
                        let article_theme = theme.clone();
                        move |_| display_sample::reading_article(article_theme.clone(), true)
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(pos.map(|(x, y)| format!("Scroll position: x = {x:.0}, y = {y:.0}"))),
            ))
            .show_in(scope)
    })
}
