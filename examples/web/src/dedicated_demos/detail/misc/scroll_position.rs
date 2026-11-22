use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let handle = scope.cage(None::<sapid::ScrollHandle>);
        let weak = scope.downgrade();
        let actions = sapid::group()
            .theme(theme.clone())
            .justify(Justify::Center)
            .gap(8.)
            .fill(
                [
                    ("Scroll to bottom", 1.),
                    ("Scroll to center", 0.5),
                    ("Scroll to top", 0.),
                ]
                .into_iter()
                .map(|(label, y)| {
                    let weak = weak.clone();
                    sapid::button()
                        .theme(theme.clone())
                        .label(label)
                        .on_press(move || {
                            if let Some(current) = handle.get()
                                && let Err(error) = current.scroll_to_fraction(0., y)
                                && let Some(scope) = weak.upgrade()
                            {
                                scope.report_error(error)
                            }
                        })
                })
                .collect::<Vec<_>>(),
            );
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Center)
            .gap(16.)
            .fill((
                sapid::scroll_area("positioned-reading")
                    .theme(theme.clone())
                    .name("Charizard reading area")
                    .on_ready(move |ready| handle.revise(|h| *h = Some(ready)))
                    .style(
                        Style::new()
                            .width(Length::Px(300.))
                            .max_width(Limit::Percent(100.))
                            .height(Length::Px(200.)),
                    )
                    .content({
                        let article_theme = theme.clone();
                        move |_| display_sample::reading_article(article_theme.clone(), false)
                    }),
                actions,
            ))
            .show_in(scope)
    })
}
