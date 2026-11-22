use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage("one".to_owned());
        let palette = theme.clone();
        let grow = scope.cage(false);
        let justify = scope.cage(Some("start".to_owned()));
        let preview =
            each(scope.bond(move || vec![grow.get()]))
                .key(Clone::clone)
                .children(move |e| {
                    sapid::tabs("aligned-tabs")
                        .theme(palette.clone())
                        .name("Tab alignment")
                        .grow(e.value.get())
                        .class(justify.map(|v| {
                            format!("site-tabs-justify-{}", v.as_deref().unwrap_or("start"))
                        }))
                        .value(selected)
                        .on_value_change(move |s| selected.revise(|v| *v = s))
                        .tab(sapid::tab("one").label("First tab"))
                        .tab(sapid::tab("two").label("Second tab"))
                        .tab(sapid::tab("three").label("Third tab"))
                });
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("Grow", grow, &theme),
                    controls::select(
                        "tabs-justify",
                        "Justify",
                        justify,
                        &[
                            ("start", "Flex-start"),
                            ("center", "Center"),
                            ("end", "Flex-end"),
                            ("between", "Space-between"),
                        ],
                        &theme,
                    ),
                ],
            ));
        panel.show_in(scope)
    })
}
