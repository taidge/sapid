use glory::prelude::*;

const PARTS: &[(&str, &str, &str)] = &[
    ("root", "Root element", ".sapid-pagination"),
    (
        "control",
        "Control element: items, next/previous, first/last buttons",
        ".sapid-page-button",
    ),
    (
        "dots",
        "Dots icon wrapper",
        ".sapid-page-slot > :last-child:not([hidden])",
    ),
    (
        "items",
        "Wrapper around page number controls, used with `layout=\"responsive\"`",
        ".sapid-page-slot",
    ),
    (
        "label",
        "Compact label element displayed in narrow containers with `layout=\"responsive\"`",
        ".site-pages-compact-label",
    ),
];

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = widget(move |scope| {
        let theme = theme.clone();
        let page = scope.cage(1_u64);
        (sapid::stack()
            .theme(theme.clone())
            .class("site-responsive-pages-demo")
            .gap(6.)
            .fill((
                sapid::pagination("styles-pages")
                    .theme(theme)
                    .name("Pagination styles")
                    .page(page)
                    .total(Some(10))
                    .on_page_change(move |next| page.revise(|v| *v = next)),
                text()
                    .class("site-pages-compact-label")
                    .content(page.map(|p| format!("{p} / 10"))),
            )))
        .show_in(scope)
    });
    crate::showcase_style_explorer::explorer("pagination-styles", Box::new(preview), PARTS)
}
