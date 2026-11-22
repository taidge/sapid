use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let variant = scope.cage(Some("light".to_owned()));
        let color = scope.cage("Blue".to_owned());
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(variant.get(), color.get())]))
            .key(Clone::clone)
            .children(move |_| {
                let theme = palette.clone();
                let accent = controls::accent(theme.clone(), &color.get()).accent;
                let filled = variant.get().as_deref() == Some("filled");
                let ink = if filled {
                    theme.contrasting_foreground(accent, theme.on_accent)
                } else {
                    accent
                };
                sapid::empty_state()
                    .theme(theme.clone())
                    .title("No notifications")
                    .description("Your notifications will appear here.")
                    .illustration(
                        glory::ui::graphic()
                            .drawing(sapid::Icon::Info.drawing(ink))
                            .style(Style {
                                background: Some(if filled {
                                    accent
                                } else {
                                    Color(accent.0, accent.1, accent.2, 32)
                                }),
                                ..Style::new()
                                    .width(Length::Px(48.))
                                    .height(Length::Px(48.))
                                    .padding(Edges::all(12.))
                                    .radius(24.)
                            }),
                    )
            });
        view()
            .class(
                "site-configurator site-display-configurator site-empty-variant-play-configurator",
            )
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::option_segments(
                        "empty_variant_play-variant",
                        "Variant",
                        variant,
                        &[("filled", "Filled"), ("light", "Light")],
                        &theme,
                    ),
                    controls::color_choices(color, &theme),
                ],
            ))
            .show_in(scope)
    })
}
