use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let attached = scope.cage(true);
        let pair = [false, true].map(|pinned| {
            let shown = attached.map(move |v| pinned || *v);
            sapid::popover(format!("detach-popover-{pinned}"))
                .theme(theme.clone())
                .preserve_focus()
                .open(shown)
                .class("site-ref-detach-popover")
                .label("Toggle popover")
                .title("Information")
                .width(sapid::PopoverWidth::Target)
                .content(move || {
                    text().content(if pinned {
                        "This popover dropdown is visible when detached"
                    } else {
                        "This popover dropdown is hidden when detached"
                    })
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
