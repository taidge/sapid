use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::TourContext::with_spotlight(scope, "custom-content")?;
        let open = scope.cage(false);
        let current = scope.cage(0usize);
        let preview = sapid::group()
            .theme(theme.clone())
            .class("site-reference-actions")
            .fill((
                sapid::tour_target(&context, "start").fill(
                    sapid::button()
                        .theme(theme.clone())
                        .label("Start tour")
                        .on_press(move || {
                            current.revise(|s| *s = 0);
                            open.revise(|s| *s = true)
                        }),
                ),
                sapid::tour_target(&context, "settings").fill(
                    sapid::button()
                        .theme(theme.clone())
                        .label("Settings")
                        .variant(sapid::ButtonVariant::Light),
                ),
            ));
        let panel = sapid::tour(&context, "custom-content")
            .theme(theme.clone())
            .compact_navigation(true)
            .navigation_labels("Back", "Next", "Close", "Skip")
            .step_counter(|step, total| format!("{step} of {total}"))
            .open(open)
            .current(current)
            .steps([
                sapid::tour_step(
                    "start",
                    "Welcome",
                    "Click here to get started with the app.",
                ),
                sapid::tour_step("settings", "Settings", "Configure your preferences here."),
            ])
            .on_current_change(move |v| current.revise(|old| *old = v))
            .on_open_change(move |v| open.revise(|old| *old = v));
        group().fill((preview, panel)).show_in(scope)
    })
}
