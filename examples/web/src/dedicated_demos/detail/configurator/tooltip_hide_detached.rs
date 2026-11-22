use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let attached = scope.cage(true);
        let pair = [false, true].map(|pinned| {
            let shown = attached.map(move |v| pinned || *v);
            sapid::tooltip(format!("detach-tooltip-{pinned}"))
                .theme(theme.clone())
                .open(shown)
                .class("site-ref-detach-tooltip")
                .text(if pinned {
                    "Visible when detached"
                } else {
                    "Hidden when detached"
                })
                .placement(glory::render::LayerPlacement::Anchor(
                    glory::render::AnchorPlacement::new(
                        glory::render::AnchorSide::Bottom,
                        glory::render::AnchorAlign::Center,
                    ),
                ))
                .target(move |t| {
                    t.button(glory::ui::button().label(if pinned {
                        "Stays visible"
                    } else {
                        "Hides when detached"
                    }))
                })
        });
        view()
            .class("site-ref-detach-viewport")
            .on(glory::render::EventKind::Scroll, move |event| {
                if let glory::render::Event::Scroll { x, y, .. } = event {
                    let next = x < 120. && y < 65.;
                    if next != attached.get_untracked() {
                        attached.revise(|v| *v = next);
                    }
                }
            })
            .fill(
                view()
                    .class("site-ref-detach-content")
                    .fill(pair.into_iter().collect::<Vec<_>>()),
            )
            .show_in(scope)
    })
}
