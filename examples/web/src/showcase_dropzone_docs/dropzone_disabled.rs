use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let disabled = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                super::image_dropzone::image_dropzone(theme.clone()).enabled(disabled.map(|v| !*v)),
                sapid::switch()
                    .theme(theme.clone())
                    .label("Disabled")
                    .checked(disabled)
                    .on_checked_change(move |v| disabled.revise(|state| *state = v)),
            ))
            .show_in(scope)
    })
}
