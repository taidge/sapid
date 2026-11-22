use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let row = |amount: u8| {
        let label = if amount == 1 {
            "1 boundary (default)".to_owned()
        } else {
            format!("{amount} boundaries")
        };
        sapid::stack().theme(theme.clone()).gap(10.).fill((
            text().content(label),
            sapid::pagination(format!("ref-boundaries-{amount}"))
                .theme(theme.clone())
                .name(format!("{amount} boundaries"))
                .total(Some(20))
                .default_page(10)
                .boundaries(amount)
                .with_edges(false),
        ))
    };
    sapid::stack()
        .theme(theme.clone())
        .class("site-ref-pages-examples")
        .gap(32.)
        .fill((row(1), row(2), row(3)))
}
