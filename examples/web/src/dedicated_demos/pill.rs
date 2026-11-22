use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let visible = scope.cage(true);
        let removable = scope.cage(false);
        let size = scope.cage(2.);
        let palette = theme.clone();
        let pill = each(scope.bond(move || vec![(size.get() as usize, removable.get())]))
            .key(Clone::clone)
            .children(move |item| {
                let (index, with_button) = item.value.get();
                let pill = sapid::pill()
                    .theme(palette.clone())
                    .size(controls::step_size(index as f64))
                    .content("React")
                    .visible(visible);
                if with_button {
                    pill.removable("Remove React tag")
                        .on_remove(move || visible.revise(|v| *v = false))
                } else {
                    pill
                }
            });
        let restore_theme = theme.clone();
        let preview = sapid::group().theme(theme.clone()).gap(12.).fill((
            pill,
            when(visible.map(|shown| !*shown), move || {
                sapid::button()
                    .theme(restore_theme.clone())
                    .label("Restore React tag")
                    .on_press(move || visible.revise(|v| *v = true))
            }),
        ));
        view()
            .class("site-configurator site-field-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::field_scale("pill-size".into(), "Size", size, &theme),
                    controls::switch("With remove button", removable, &theme),
                ],
            ))
            .show_in(scope)
    })
}
