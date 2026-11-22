use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage("Blue".to_owned());
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![color.get()]))
            .key(Clone::clone)
            .children(move |entry| {
                let mut theme = controls::accent(palette.clone(), &entry.value.get());
                theme.surface = theme.accent;
                theme.foreground = theme.contrasting_foreground(theme.accent, theme.on_accent);
                sapid::tooltip("colored-tip")
                    .theme(theme)
                    .label("Hover to preview color")
                    .text("A colored tooltip")
            });
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::color_choices(color, &theme)],
            ));
        panel.show_in(scope)
    })
}
