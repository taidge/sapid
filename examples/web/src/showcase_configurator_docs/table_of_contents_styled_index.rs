use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let ctx = sapid::toc_context(scope, "toc-styled-index")?;
        let mut ink = theme.clone();
        ink.accent = Color(255, 224, 102, 255);
        ink.on_accent = Color(0, 0, 0, 255);
        let engaged = scope.cage(false);
        let active = ctx.current(scope);
        let entries = ctx.entries(scope);
        let selected = scope.bond(move || {
            if engaged.get() {
                active.get()
            } else {
                entries.with(|rows| rows.get(7).map(|r| r.id.clone()))
            }
        });
        let article_theme = theme.clone();
        let toc = sapid::table_of_contents(ctx.clone(), "index-styled-index")
            .theme(ink)
            .size(sapid::Size::Small)
            .class("site-ref-toc-lime")
            .current(selected)
            .variant(sapid::NavLinkVariant::Filled);
        let doc = sapid::toc_document(ctx, "article-styled-index")
            .theme(theme.clone())
            .name("Guide headings")
            .style(Style::new().height(Length::Px(130.)))
            .content(move |ctx| {
                sapid::stack().theme(article_theme.clone()).gap(40.).fill(
                    [
                        "TableOfContents",
                        "Usage",
                        "use-scroll-spy options",
                        "Pass props to controls",
                        "Initial data",
                        "Depth offset",
                        "autoContrast",
                        "Styles API",
                        "Reinitialize",
                    ]
                    .into_iter()
                    .enumerate()
                    .map(|(n, title)| {
                        sapid::toc_heading(ctx.clone(), format!("toc-styled-index-{n}"), 2)
                            .theme(article_theme.clone())
                            .order(n as u32)
                            .content(title)
                    })
                    .collect::<Vec<_>>(),
                )
            });
        view()
            .class("site-ref-toc-frame")
            .on(glory::render::EventKind::FocusWithin, move |e| {
                if let glory::render::Event::FocusWithin { focused: true, .. } = e {
                    engaged.revise(|v| *v = true);
                }
            })
            .fill((toc, doc))
            .show_in(scope)
    })
}
