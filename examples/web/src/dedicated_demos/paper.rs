use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let shadow = scope.cage(Some("small".to_owned()));
        let radius = scope.cage(10_f64);
        let border = scope.cage(true);
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(shadow.get(), border.get())]))
            .key(Clone::clone)
            .children(move |_| {
                let theme = palette.clone();

                sapid::paper()
                    .theme(theme.clone())
                    .bordered(border.get())
                    .shadow(match shadow.get().as_deref() {
                        Some("small") => sapid::ShadowSize::Small,
                        Some("medium") => sapid::ShadowSize::Medium,
                        Some("large") => sapid::ShadowSize::Large,
                        _ => sapid::ShadowSize::None,
                    })
                    .style(radius.map(|r| Style::new().padding(Edges::all(20.)).radius(*r)))
                    .fill(
                        sapid::stack().theme(theme.clone()).gap(10.).fill((
                            sapid::title(3)
                                .theme(theme.clone())
                                .weight(400.)
                                .content("A place for a useful note"),
                            sapid::text().theme(theme.clone()).content(TEXT_1),
                        )),
                    )
            });
        view()
            .class("site-configurator site-display-configurator site-paper-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::select(
                        "paper-shadow",
                        "Shadow",
                        shadow,
                        &[
                            ("none", "None"),
                            ("small", "Small"),
                            ("medium", "Medium"),
                            ("large", "Large"),
                        ],
                        &theme,
                    ),
                    controls::range("paper-radius", "Radius", radius, 0., 24., 1., &theme),
                    controls::switch("With border", border, &theme),
                ],
            ))
            .show_in(scope)
    })
}

const TEXT_1: &str = "Paper supplies the surface. Your own widgets supply the content and actions.";
