use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(true);
        let size = scope.cage(Some("md".to_owned()));
        let variant = scope.cage("subtle".to_owned());
        let palette = theme.clone();
        let close = each(scope.bond(move || if open.get() { vec![size.get()] } else { vec![] }))
            .key(Clone::clone)
            .children(move |_| {
                sapid::close_button()
                    .theme(palette.clone())
                    .size(controls::size_value(size.get().as_deref().unwrap_or("md")))
                    .name("Close notification")
                    .variant(sapid::ActionIconVariant::Subtle)
                    .class(variant.map(|v| {
                        if v == "transparent" {
                            "site-close-transparent".to_owned()
                        } else {
                            String::new()
                        }
                    }))
                    .on_press(move || open.revise(|v| *v = false))
            });
        let restore_theme = theme.clone();
        let preview = view().fill((
            close,
            when(open.map(|v| !*v), move || {
                sapid::button()
                    .theme(restore_theme.clone())
                    .label("Restore notification")
                    .on_press(move || open.revise(|v| *v = true))
            }),
        ));
        view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::size_slider(size, &theme),
                    controls::segmented_choice(
                        "close-variant",
                        "Variant",
                        variant,
                        &[("transparent", "Transparent"), ("subtle", "Subtle")],
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
