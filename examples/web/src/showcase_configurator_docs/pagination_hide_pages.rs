use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let page = scope.cage(1_u64);
        view()
            .class("site-ref-pager-summary")
            .fill((
                text().content(page.map(|p| {
                    format!(
                        "Showing {} – {} of 145",
                        (p - 1) * 10 + 1,
                        (p * 10).min(145)
                    )
                })),
                sapid::pagination("counted-docs-pages")
                    .theme(theme.clone())
                    .total(Some(15))
                    .page(page)
                    .with_edges(false)
                    .on_page_change(move |v| page.revise(|current| *current = v)),
            ))
            .show_in(scope)
    })
}
