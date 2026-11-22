use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-ref-page-pair")
        .fill((
            sapid::pagination("links-standard")
                .theme(theme.clone())
                .total(Some(10))
                .page_href(|page| format!("#page-{page}")),
            sapid::pagination("links-compound")
                .theme(theme)
                .total(Some(10))
                .page_href(|page| format!("#page-{page}")),
        ))
}
