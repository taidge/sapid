use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let kind = scope.cage(Some("unordered".to_owned()));
        let size = scope.cage(Some("md".to_owned()));
        let padding = scope.cage(false);
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(kind.get(), size.get())]))
            .key(Clone::clone)
            .children(move |_| {
                let theme = palette.clone();

                sapid::text_list()
                    .theme(theme.clone())
                    .name("Project launch checklist")
                    .ordered(kind.get().as_deref() == Some("ordered"))
                    .size(controls::size_value(size.get().as_deref().unwrap_or("md")))
                    .style(padding.map(|on| {
                        Style::new().padding(Edges {
                            left: if *on { 36. } else { 20. },
                            ..Edges::all(0.)
                        })
                    }))
                    .items(
                        [
                            "Clone or download the repository from GitHub",
                            "Install dependencies with yarn",
                            "Start the development server",
                            "Run tests to check your changes",
                            "Submit a pull request when the changes are ready",
                        ]
                        .into_iter()
                        .map(|label| {
                            sapid::text_list_item()
                                .fill(sapid::text().theme(theme.clone()).content(label))
                        }),
                    )
            });
        view()
            .class("site-configurator site-display-configurator site-list-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::option_segments(
                        "list-kind",
                        "Type",
                        kind,
                        &[("unordered", "Unordered"), ("ordered", "Ordered")],
                        &theme,
                    ),
                    controls::size_slider(size, &theme),
                    controls::switch("With padding", padding, &theme),
                ],
            ))
            .show_in(scope)
    })
}
