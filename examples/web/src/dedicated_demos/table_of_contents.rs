use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::toc_context(scope, "getting-started")?;
        let article_theme = theme.clone();
        let palette = theme.clone();
        let color = scope.cage("Blue".to_owned());
        let size = scope.cage(Some("md".to_owned()));
        let radius = scope.cage(8.);
        let variant = scope.cage("filled".to_owned());
        let index_context = context.clone();
        let preview = each(
            scope.bond(move || vec![(variant.get(), color.get(), size.get(), radius.get() as u8)]),
        )
        .key(Clone::clone)
        .children(move |r| {
            let (variant, color, size, radius) = r.value.get();
            let mut theme = controls::accent(palette.clone(), &color);
            theme.radius = f64::from(radius);
            let ink = theme.accent;
            theme.accent_soft = Color(ink.0, ink.1, ink.2, 32);
            theme.accent_text = ink;
            sapid::table_of_contents(index_context.clone(), "guide-index")
                .theme(theme)
                .size(controls::size_value(size.as_deref().unwrap_or("md")))
                .variant(match variant.as_str() {
                    "filled" => sapid::NavLinkVariant::Filled,
                    "light" => sapid::NavLinkVariant::Light,
                    _ => sapid::NavLinkVariant::Subtle,
                })
                .name("Documentation sections")
                .class("site-doc-toc")
        });
        sapid::stack()
            .theme(theme.clone())
            .gap(20.)
            .fill((
                view().class("site-configurator").fill(controls::playground(
                    Box::new(preview),
                    vec![
                        controls::segmented_choice(
                            "toc-variant",
                            "Variant",
                            variant,
                            &[("filled", "Filled"), ("light", "Light"), ("none", "None")],
                            &theme,
                        ),
                        controls::color_choices(color, &theme),
                        controls::size_slider(size, &theme),
                        controls::pixel_scale(
                            "table_of_contents-radius",
                            "Radius",
                            radius,
                            [2., 4., 8., 16., 32.],
                            &theme,
                        ),
                    ],
                )),
                sapid::toc_document(context, "guide-article")
                    .theme(theme.clone())
                    .name("Component documentation")
                    .offset(12.)
                    .style(Style::new().height(Length::Px(300.)))
                    .content(move |context| {
                        sapid::stack().theme(article_theme.clone()).gap(24.).fill(
                            Vec::from([
                                (
                                    "usage",
                                    "Usage",
                                    "The index tracks the headings in this article.",
                                ),
                                (
                                    "scroll-spy",
                                    "use-scroll-spy options",
                                    "Follow the active heading as the article scrolls.",
                                ),
                                (
                                    "control-props",
                                    "Pass props to controls",
                                    "Configure how each link appears.",
                                ),
                                (
                                    "initial-data",
                                    "Initial data",
                                    "Register headings in document order.",
                                ),
                                (
                                    "depth-offset",
                                    "Depth offset",
                                    "Indent headings by their document level.",
                                ),
                                (
                                    "auto-contrast",
                                    "autoContrast",
                                    "Adjust the color contrast of the active item.",
                                ),
                                (
                                    "styles-api",
                                    "Styles API",
                                    "Style the root, controls and labels independently.",
                                ),
                                (
                                    "reinitialize",
                                    "Reinitialize",
                                    "Refresh the index when the document changes.",
                                ),
                            ])
                            .into_iter()
                            .enumerate()
                            .map(|(index, (key, title, description))| {
                                section(
                                    &context,
                                    &article_theme,
                                    key,
                                    title,
                                    2,
                                    index as u32,
                                    description,
                                )
                            })
                            .collect::<Vec<_>>(),
                        )
                    }),
            ))
            .show_in(scope)
    })
}
fn section(
    context: &sapid::TocContext,
    theme: &sapid::Theme,
    key: &str,
    title: &str,
    level: u8,
    order: u32,
    description: &str,
) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::toc_heading(context.clone(), key, level)
            .theme(theme.clone())
            .order(order)
            .content(title.to_owned())
            .style(Style::new().font_weight(400.)),
        sapid::text()
            .theme(theme.clone())
            .content(description.to_owned()),
    ))
}
