use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                super::image_dropzone::image_dropzone(theme.clone()).loading(loading),
                sapid::switch()
                    .theme(theme.clone())
                    .label("Loading")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|state| *state = v)),
            ))
            .show_in(scope)
    })
}
