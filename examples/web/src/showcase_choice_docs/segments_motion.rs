use glory::prelude::*;

// Add these rules to your application stylesheet:
// .report-smooth { --sp-segment-indicator: var(--sp-accent); --sp-segment-selected-text: var(--sp-on-accent); --sp-segment-duration: 500ms; --sp-segment-easing: linear; }
// .report-instant { --sp-segment-duration: 0ms; }
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let policy = |key, label, class| {
        sapid::segmented_control(key)
            .theme(theme.clone())
            .label(label)
            .class(class)
            .default_value("draft")
            .segments([
                sapid::segment("draft").label("Draft"),
                sapid::segment("review").label("In review"),
                sapid::segment("published").label("Published"),
            ])
    };
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        policy("report-animated", "500 ms linear", "report-smooth"),
        policy("report-static", "Instant", "report-instant"),
    ))
}
