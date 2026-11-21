use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let checked = scope.cage(true);
        let label = scope.cage("Notifications".to_owned());
        let readonly = scope.cage(false);
        let disabled = scope.cage(false);
        let size = scope.cage(Some("md".to_owned()));
        let marks = scope.cage(true);
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(size.get(), marks.get())]))
            .key(Clone::clone)
            .children(move |_| {
                sapid::switch()
                    .theme(palette.clone())
                    .size(controls::size_value(size.get().as_deref().unwrap_or("md")))
                    .label(label)
                    .with_marks(marks.get())
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
                    controls::switch("Show marks", marks, &theme),
                    controls::switch("Read only", readonly, &theme),
                    controls::switch("Disabled", disabled, &theme),
                ],
            ))
            .show_in(scope)
    })
}
