use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let state = scope.cage(sapid::CopyState::Idle);
        let feedback_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .class("site-standalone-action")
            .gap(12.)
            .fill((
                sapid::copy_button()
                    .theme(theme.clone())
                    .style(Style {
                        background: Some(theme.accent),
                        color: Some(theme.on_accent),
                        border_color: Some(theme.accent),
                        ..Style::new().radius(4.)
                    })
                    .source("https://example.com/invite/team")
                    .label("Copy url")
                    .copied_label("Copied")
                    .failed_label("Could not copy link")
                    .state_ref(state),
                when(state.map(|v| *v != sapid::CopyState::Idle), move || {
                    sapid::text()
                        .theme(feedback_theme.clone())
                        .content(state.map(|v| format!("Clipboard result: {v:?}")))
                }),
            ))
            .show_in(scope)
    })
}
