use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::TourContext::with_spotlight(scope, "tooltip-position")?;
        let open = scope.cage(false);
        let current = scope.cage(0usize);
        let preview = view().class("site-ref-tour-position").fill(
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
        let panel = sapid::tour(&context, "tooltip-position")
            .theme(theme.clone())
            .compact_navigation(true)
            .navigation_labels("Back", "Next", "Close", "Skip")
            .step_counter(|step, total| format!("{step} of {total}"))
            .open(open)
            .current(current)
            .anchored_step(
                sapid::tour_step("bottom", "Bottom", "Tooltip positioned at the bottom."),
                "start",
                glory::render::AnchorPlacement::new(
                    glory::render::AnchorSide::Bottom,
                    glory::render::AnchorAlign::Center,
                ),
            )
            .anchored_step(
                sapid::tour_step("top", "Top", "Tooltip positioned at the top."),
                "start",
                glory::render::AnchorPlacement::new(
                    glory::render::AnchorSide::Top,
                    glory::render::AnchorAlign::Center,
                ),
            )
            .anchored_step(
                sapid::tour_step("left", "Left", "Tooltip positioned at the left."),
                "start",
                glory::render::AnchorPlacement::new(
                    glory::render::AnchorSide::Left,
                    glory::render::AnchorAlign::Center,
                ),
            )
            .anchored_step(
                sapid::tour_step("right", "Right", "Tooltip positioned at the right."),
                "start",
                glory::render::AnchorPlacement::new(
                    glory::render::AnchorSide::Right,
                    glory::render::AnchorAlign::Center,
                ),
            )
            .over_target_step(
                sapid::tour_step(
                    "center",
                    "Center",
                    "Tooltip positioned over the center of the target.",
                ),
                "start",
            )
            .on_current_change(move |v| current.revise(|old| *old = v))
            .on_open_change(move |v| open.revise(|old| *old = v));
        group().fill((preview, panel)).show_in(scope)
    })
}
