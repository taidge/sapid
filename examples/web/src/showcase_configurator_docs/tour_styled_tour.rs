use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::TourContext::with_spotlight(scope, "styled-tour")?;
        let open = scope.cage(false);
        let current = scope.cage(0usize);
        let preview = sapid::stack()
            .theme(theme.clone())
            .class("site-ref-tour-form")
            .gap(16.)
            .fill((
                sapid::group()
                    .theme(theme.clone())
                    .class("site-reference-actions")
                    .fill((
                        sapid::tour_target(&context, "first")
                            .fill(sapid::button().theme(theme.clone()).label("First target")),
                        sapid::tour_target(&context, "second").fill(
                            sapid::text_field()
                                .theme(theme.clone())
                                .name("Second target")
                                .placeholder("Second target"),
                        ),
                        sapid::tour_target(&context, "third").fill(
                            sapid::button()
                                .theme(theme.clone())
                                .label("Third target")
                                .variant(sapid::ButtonVariant::Outline),
                        ),
                    )),
                sapid::button()
                    .theme(theme.clone())
                    .label("Start tour")
                    .on_press(move || {
                        current.revise(|s| *s = 0);
                        open.revise(|s| *s = true)
                    }),
            ));
        let panel = sapid::tour(&context, "styled-tour")
            .theme(theme.clone())
            .compact_navigation(true)
            .navigation_labels("Back", "Next", "Close", "Skip")
            .step_counter(|step, total| format!("{step} of {total}"))
            .open(open)
            .current(current)
            .steps([
                sapid::tour_step(
                    "first",
                    "Step 1: Welcome",
                    "This tour demonstrates how to customize Tour styles with classNames.",
                ),
                sapid::tour_step(
                    "second",
                    "Step 2: Input",
                    "You can style every part of the tooltip independently.",
                ),
                sapid::tour_step(
                    "third",
                    "Step 3: Actions",
                    "The tooltip, navigation buttons, and counters are all customized.",
                ),
            ])
            .on_current_change(move |v| current.revise(|old| *old = v))
            .on_open_change(move |v| open.revise(|old| *old = v));
        group().fill((preview, panel)).show_in(scope)
    })
}
