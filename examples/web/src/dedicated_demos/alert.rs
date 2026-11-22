use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let visible = scope.cage(true);

        let variant = scope.cage(Some("light".to_owned()));
        let color = scope.cage("Blue".to_owned());
        let radius = scope.cage(8_f64);
        let closable = scope.cage(false);
        let title = scope.cage("Alert title".to_owned());
        let children = scope.cage("Lorem ipsum dolor sit, amet consectetur adipisicing elit. At officia quasi tempore necessitatibus placeat saepe.".to_owned());
        let palette = theme.clone();
        let preview = each(
            scope.bond(move || vec![(variant.get(), color.get(), closable.get(), title.get())]),
        )
        .key(Clone::clone)
        .children(move |_| {
            let theme = palette.clone();

            let tint = controls::accent(theme.clone(), &color.get()).accent;
            let ink = if variant.get().as_deref() == Some("filled") {
                theme.contrasting_foreground(tint, theme.on_accent)
            } else {
                tint
            };
            let alert = sapid::alert("storage-notice");
            let alert = if title.get().trim().is_empty() {
                alert
            } else {
                alert.title(title.get())
            };
            sapid::stack().theme(theme.clone()).gap(12.).fill((
                alert
                    .theme(theme.clone())
                    .optional_content(children)
                    .color(controls::accent(theme.clone(), &color.get()).accent)
                    .style(radius.map(|r| Style::new().radius(*r)))
                    .variant(match variant.get().as_deref() {
                        Some("filled") => sapid::AlertVariant::Filled,
                        Some("outline") => sapid::AlertVariant::Outline,
                        Some("transparent") => sapid::AlertVariant::Transparent,
                        _ => sapid::AlertVariant::Light,
                    })
                    .announcement(sapid::Announcement::Off)
                    .icon(
                        glory::ui::graphic()
                            .drawing(sapid::Icon::Info.drawing(ink))
                            .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                    )
                    .closable(closable.get())
                    .close_label("Dismiss storage notice")
                    .visible(visible)
                    .on_close(move || visible.revise(|v| *v = false)),
                when(visible.map(|v| !*v), move || {
                    sapid::button()
                        .theme(theme.clone())
                        .label("Restore storage notice")
                        .on_press(move || visible.revise(|v| *v = true))
                }),
            ))
        });
        view()
            .class("site-configurator site-display-configurator site-alert-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::select(
                        "alert-variant",
                        "Variant",
                        variant,
                        &[
                            ("light", "Light"),
                            ("filled", "Filled"),
                            ("outline", "Outline"),
                            ("transparent", "Transparent"),
                        ],
                        &theme,
                    ),
                    controls::color_choices(color, &theme),
                    controls::pixel_scale(
                        "alert-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                    controls::switch("With close button", closable, &theme),
                    controls::text_field("Title", title, &theme),
                    controls::text_field("Children", children, &theme),
                ],
            ))
            .show_in(scope)
    })
}
