use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("P6-01", theme)
}

#[allow(dead_code, unused_imports)]
mod focused {
    use glory::prelude::*;
    use sapid::Theme;

    pub fn demo_for(id: &str, theme: Theme) -> Box<dyn Widget> {
        match id {
            "P6-01" => {
                let trail = |key, separator, gap| {
                    sapid::breadcrumbs(key)
                        .theme(theme.clone())
                        .separator(separator)
                        .gap(gap)
                        .items([
                            sapid::breadcrumb_item("home", "Sapid").href("showcase.html"),
                            sapid::breadcrumb_item("library", "Components")
                                .href("?widget=breadcrumbs"),
                            sapid::breadcrumb_item("anchor", "Anchor").href("?widget=anchor"),
                        ])
                };
                Box::new(sapid::stack().theme(theme.clone()).gap(10.).fill((
                    trail("focused-trail", "/", 8.),
                    trail("spaced-trail", "→", 16.),
                )))
            }
            "P6-02" => Box::new(widget(move |scope| {
                let page = scope.cage(3_u64);
                sapid::pagination("focused-pages")
                    .theme(theme.clone())
                    .name("Example pages")
                    .page(page)
                    .total(Some(8))
                    .on_page_change(move |next| page.revise(|current| *current = next))
                    .show_in(scope)
            })),
            _ => unreachable!("unsupported navigation demo"),
        }
    }
}
