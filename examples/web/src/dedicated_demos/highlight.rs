use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let ignore_case = scope.cage(true);
        let whole_words = scope.cage(false);
        let color = scope.cage("Yellow".to_owned());
        let queries = scope.cage("this".to_owned());
        let content = scope.cage("Highlight This, definitely THIS and also this!".to_owned());
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                color.get(),
                queries.get(),
                content.get(),
                ignore_case.get(),
                whole_words.get(),
            )]
        }))
        .key(Clone::clone)
        .seed(move |(color, queries, content, ignore_case, whole_words)| {
            let mut theme = palette.clone();
            theme.accent_soft = controls::mark_color(&theme, &color);
            theme.accent_text = theme.contrasting_foreground(theme.accent_soft, theme.foreground);
            sapid::highlight()
                .theme(theme)
                .content(content)
                .queries(
                    queries
                        .split(',')
                        .map(str::trim)
                        .filter(|q| !q.is_empty() && q.len() <= 256)
                        .take(32)
                        .map(str::to_owned),
                )
                .ignore_ascii_case(ignore_case)
                .whole_words(whole_words)
        });
        view()
            .class("site-configurator site-display-configurator site-highlight-configurator")
            .fill(controls::playground(
                Box::new(
                    view()
                        .style(Style::new().display(Display::Block))
                        .fill(preview),
                ),
                vec![
                    controls::color_choices(color, &theme),
                    controls::text_field("Highlight", queries, &theme),
                    controls::text_field("Children", content, &theme),
                    controls::switch("Ignore ASCII case", ignore_case, &theme),
                    controls::switch("Whole words", whole_words, &theme),
                ],
            ))
            .show_in(scope)
    })
}
