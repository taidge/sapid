use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let presses = scope.cage(0);
        let loading = scope.cage(false);
        let color = scope.cage("Blue".to_owned());
        let size = scope.cage(Some("md".to_owned()));
        let radius = scope.cage(6.);
        let variant = scope.cage(Some("light".to_owned()));
        let palette = theme.clone();
        let action = each(scope.bond(move || vec![(color.get(), size.get(), variant.get())]))
            .key(Clone::clone)
            .children(move |_| {
                let mut theme = controls::accent(palette.clone(), &color.get());
                let c = theme.accent;
                theme.accent_soft = glory::style::Color(c.0, c.1, c.2, 28);
                let variant = match variant.get().as_deref() {
                    Some("filled") => sapid::ActionIconVariant::Filled,
                    Some("outline") => sapid::ActionIconVariant::Outline,
                    _ => sapid::ActionIconVariant::Light,
                };
                let ink = if variant == sapid::ActionIconVariant::Filled {
                    theme.contrasting_foreground(c, theme.on_accent)
                } else {
                    theme.accent_text
                };
                sapid::action_icon("Confirm selection")
                    .theme(theme)
                    .size(controls::size_value(size.get().as_deref().unwrap_or("md")))
                    .variant(variant)
                    .loading(loading)
                    .auto_contrast(true)
                    .style(radius.map(|r| Style::new().radius(*r)))
                    .icon(
                        glory::ui::graphic()
                            .drawing(sapid::Icon::Check.drawing(ink))
                            .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                    )
                    .on_press(move || presses.revise(|n| *n += 1))
            });
        let preview = sapid::stack().theme(theme.clone()).gap(8.).fill((
            action,
            sapid::text()
                .theme(theme.clone())
                .content(presses.map(|n| format!("Confirmed selections: {n}"))),
        ));
        view()
            .class("site-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::select(
                        "action-variant",
                        "Variant",
                        variant,
                        &[
                            ("light", "Light"),
                            ("filled", "Filled"),
                            ("outline", "Outline"),
                        ],
                        &theme,
                    ),
                    controls::size_slider(size, &theme),
                    controls::radius_slider(radius, &theme),
                    controls::color_choices(color, &theme),
                    controls::switch("Loading", loading, &theme),
                ],
            ))
            .show_in(scope)
    })
}
