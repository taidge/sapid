use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let checked = scope.cage(false);
        let label = scope.cage("I agree to the terms and conditions".to_owned());
        let readonly = scope.cage(false);
        let disabled = scope.cage(false);
        let size = scope.cage(Some("md".to_owned()));

        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(size.get(), ())]))
            .key(Clone::clone)
            .children(move |_| {
                sapid::checkbox()
                    .theme(palette.clone())
                    .size(controls::size_value(size.get().as_deref().unwrap_or("md")))
                    .label(label)
                    .checked(checked)
                    .readonly(readonly)
                    .enabled(disabled.map(|v| !*v))
                    .on_checked_change(move |next| checked.revise(|v| *v = next))
            });
        view()
            .class("site-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::text_field("Label", label, &theme),
                    controls::size_slider(size, &theme),
                    controls::switch("Read only", readonly, &theme),
                    controls::switch("Disabled", disabled, &theme),
                ],
            ))
            .show_in(scope)
    })
}
