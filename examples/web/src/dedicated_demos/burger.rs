use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let size = scope.cage(Some("md".to_owned()));
        let palette = theme.clone();
        let burger = each(scope.bond(move || vec![size.get()]))
            .key(Clone::clone)
            .children(move |_| {
                sapid::burger()
                    .theme(palette.clone())
                    .size(controls::size_value(size.get().as_deref().unwrap_or("md")))
                    .open(open)
                    .on_open_change(move |next| open.revise(|value| *value = next))
            });
        let preview = burger;
        view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::size_slider(size, &theme)],
            ))
            .show_in(scope)
    })
}
