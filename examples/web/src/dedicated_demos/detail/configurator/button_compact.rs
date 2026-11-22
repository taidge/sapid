use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let size = scope.cage(Some("md".to_owned()));
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![size.get()]))
            .key(Clone::clone)
            .children(move |entry| {
                let selection = entry.value.get();
                let label = selection.as_deref().unwrap_or("md");
                let size = controls::size_value(label);
                sapid::group().theme(palette.clone()).gap(16.).fill((
                    sapid::button()
                        .theme(palette.clone())
                        .size(size)
                        .label(format!("Regular {label}")),
                    sapid::button()
                        .theme(palette.clone())
                        .size(size)
                        .compact(true)
                        .label(format!("Compact {label}")),
                ))
            });
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::size_slider(size, &theme)],
            ));
        panel.show_in(scope)
    })
}
