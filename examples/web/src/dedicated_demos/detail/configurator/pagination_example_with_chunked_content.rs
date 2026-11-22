use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let page = scope.cage(1u64);
        let theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .class("site-reference-chunked")
            .gap(8.)
            .fill((
                each(scope.bond(move || {
                    let start = (page.get() - 1) * 5;
                    (start..start + 5).collect::<Vec<_>>()
                }))
                .key(Clone::clone)
                .children(|row| {
                    let id = row.value.get();
                    text().content(format!("id: {id}, name: item-{id:02}"))
                }),
                sapid::pagination("chunked-pagination")
                    .theme(theme)
                    .total(Some(6))
                    .page(page)
                    .on_page_change(move |n| page.revise(|v| *v = n)),
            ))
            .show_in(scope)
    })
}
