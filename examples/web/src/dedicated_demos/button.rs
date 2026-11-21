use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let count = scope.cage(0);
        let compact = scope.cage(false);
        let loading = scope.cage(false);
        let preview = sapid::stack().theme(theme.clone()).gap(12.).fill((
            sapid::button()
                .theme(theme.clone())
                .label("Filled button")
                .compact(compact)
                .loading(loading)
                .on_press(move || count.revise(|v| *v += 1)),
            sapid::text()
                .theme(theme.clone())
                .content(count.map(|v| format!("Button presses: {v}"))),
        ));
        view()
            .class("site-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("Compact", compact, &theme),
                    controls::switch("Loading", loading, &theme),
                ],
            ))
            .show_in(scope)
    })
}
