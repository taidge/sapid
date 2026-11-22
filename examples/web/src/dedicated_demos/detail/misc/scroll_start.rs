use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let ready = scope.cage(None::<sapid::ScrollHandle>);
        let mut positioned = false;
        let weak = scope.downgrade();
        let article_theme = theme.clone();
        let area = sapid::scroll_area("start-reading")
            .theme(theme.clone())
            .name("Charizard reading area")
            .style(
                Style::new()
                    .width(Length::Px(300.))
                    .max_width(Limit::Percent(100.))
                    .height(Length::Px(200.)),
            )
            .on_ready(move |handle| ready.revise(|saved| *saved = Some(handle)))
            .on_metrics_change(move |m| {
                if !positioned
                    && m.content_height > m.viewport_height
                    && m.viewport_height > 0.
                    && let Some(handle) = ready.get()
                {
                    positioned = true;
                    if let Err(error) = handle.scroll_to(0., 250.)
                        && let Some(scope) = weak.upgrade()
                    {
                        scope.report_error(error);
                    }
                }
            })
            .content(move |_| display_sample::reading_article(article_theme.clone(), false));
        sapid::group()
            .theme(theme.clone())
            .justify(Justify::Center)
            .fill(area)
            .show_in(scope)
    })
}
