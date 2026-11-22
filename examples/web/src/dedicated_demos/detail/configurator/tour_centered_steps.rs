use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::TourContext::with_spotlight(scope, "centered-steps")?;
        let open = scope.cage(false);
        let current = scope.cage(0usize);
        let preview = view().class("site-ref-tour-centered").fill(
            sapid::tour_target(&context, "start").fill(
                sapid::button()
                    .theme(theme.clone())
                    .label("Start tour")
                    .on_press(move || {
                        current.revise(|s| *s = 0);
                        open.revise(|s| *s = true)
                    }),
            ),
        );
        let panel = sapid::tour(&context, "centered-steps")
            .theme(theme.clone())
            .compact_navigation(true)
            .navigation_labels("Back", "Next", "Close", "Skip")
            .step_counter(|step, total| format!("{step} of {total}"))
            .open(open)
            .current(current)
            .centered_step(
                "Welcome",
                "This step has no target and is displayed in the center of the screen.",
            )
            .step(sapid::tour_step(
                "start",
                "Target step",
                "This step highlights the button.",
            ))
            .centered_step(
                "All done",
                "This final step is also centered with no target.",
            )
            .on_current_change(move |v| current.revise(|old| *old = v))
            .on_open_change(move |v| open.revise(|old| *old = v));
        group().fill((preview, panel)).show_in(scope)
    })
}
