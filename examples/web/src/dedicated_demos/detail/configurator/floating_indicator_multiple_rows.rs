use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(Some("center".to_owned()));
        let ink = theme.clone();
        sapid::floating_indicator("direction-pad")
            .theme(theme.clone())
            .class("site-ref-direction-pad")
            .active(active)
            .content(move |ctx| {
                view().class("site-ref-direction-grid").fill(
                    [
                        ("up-left", "↖"),
                        ("up", "↑"),
                        ("up-right", "↗"),
                        ("left", "←"),
                        ("center", "○"),
                        ("right", "→"),
                        ("down-left", "↙"),
                        ("down", "↓"),
                        ("down-right", "↘"),
                    ]
                    .map(|(key, arrow)| {
                        sapid::floating_indicator_target(ctx.clone(), key).fill(
                            sapid::button()
                                .theme(ink.clone())
                                .class("site-ref-direction-button")
                                .variant(sapid::ButtonVariant::Subtle)
                                .name(key)
                                .label(arrow)
                                .pressed(active.map(move |v| v.as_deref() == Some(key)))
                                .on_press(move || active.revise(|s| *s = Some(key.into()))),
                        )
                    })
                    .into_iter()
                    .collect::<Vec<_>>(),
                )
            })
            .show_in(scope)
    })
}
