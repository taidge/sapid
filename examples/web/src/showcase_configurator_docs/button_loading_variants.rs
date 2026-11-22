use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .class("site-ref-loading-row")
            .fill((
                sapid::group().theme(theme.clone()).gap(12.).fill(
                    [
                        (sapid::ButtonVariant::Primary, "Filled button"),
                        (sapid::ButtonVariant::Light, "Light button"),
                        (sapid::ButtonVariant::Outline, "Outline button"),
                    ]
                    .map(|(v, n)| {
                        sapid::button()
                            .theme(theme.clone())
                            .variant(v)
                            .label(n)
                            .loading(loading)
                    })
                    .into_iter()
                    .collect::<Vec<_>>(),
                ),
                sapid::switch()
                    .theme(theme.clone())
                    .label("Loading state")
                    .checked(loading)
                    .on_checked_change(move |next| loading.revise(|v| *v = next)),
            ))
            .show_in(scope)
    })
}
