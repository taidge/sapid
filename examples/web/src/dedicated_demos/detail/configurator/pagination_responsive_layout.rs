use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let page = scope.cage(1_u64);
        view()
            .class("site-ref-resizable-pages")
            .fill((
                sapid::pagination("responsive-docs-pages")
                    .theme(theme.clone())
                    .page(page)
                    .total(Some(20))
                    .with_edges(false)
                    .on_page_change(move |v| page.revise(|current| *current = v)),
                text()
                    .class("site-ref-responsive-page-counter")
                    .content(page.map(|value| format!("Page {value} of 20"))),
            ))
            .show_in(scope)
    })
}
