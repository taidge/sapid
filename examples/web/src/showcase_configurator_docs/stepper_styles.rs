use glory::prelude::*;

const PARTS: &[(&str, &str, &str)] = &[
    ("root", "Root element", ".sapid-stepper"),
    ("steps", "Steps controls wrapper", ".sapid-stepper-headers"),
    (
        "separator",
        "Separator line between step controls",
        ".sapid-stepper-connector",
    ),
    (
        "verticalSeparator",
        "Vertical separator line between step controls",
        ".sapid-stepper-vertical-connector",
    ),
    (
        "content",
        "Current step content wrapper",
        ".sapid-stepper-panel",
    ),
    (
        "stepWrapper",
        "Wrapper for the step icon and separator",
        ".sapid-stepper-step",
    ),
    ("step", "Step control button", ".sapid-stepper-trigger"),
    ("stepIcon", "Step icon wrapper", ".sapid-stepper-trigger"),
    (
        "stepCompletedIcon",
        "Completed step icon, rendered within stepIcon",
        ".sapid-stepper-completed-icon",
    ),
    (
        "stepIconContent",
        "Step icon content wrapper for non-completed steps, rendered within stepIcon",
        ".sapid-stepper-icon-content",
    ),
    (
        "stepBody",
        "Contains stepLabel and stepDescription",
        ".sapid-stepper-text",
    ),
    ("stepLabel", "Step label", ".sapid-stepper-label"),
    (
        "stepDescription",
        "Step description",
        ".sapid-stepper-description",
    ),
    ("stepLoader", "Step loader", ".sapid-button-loader"),
];

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = widget(move |scope| {
        let theme = theme.clone();
        let active = scope.cage(1usize);
        let vertical = scope.cage(false);
        let workflow_theme = theme.clone();
        (sapid::stack().theme(theme.clone()).gap(20.).fill((
            each(scope.bond(move || vec![vertical.get()]))
                .key(Clone::clone)
                .children(move |item| {
                    sapid::stepper("styles-workflow")
                        .theme(workflow_theme.clone())
                        .name("Account setup")
                        .orientation(if item.value.get() {
                            glory::render::Orientation::Vertical
                        } else {
                            glory::render::Orientation::Horizontal
                        })
                        .active(active)
                        .on_step_change(move |n| active.revise(|v| *v = n))
                        .steps([
                            sapid::step("account")
                                .label("First step")
                                .description("Create an account")
                                .loading(active.map(|v| *v < 2))
                                .content(|| text().content("Step 1 content: Create an account")),
                            sapid::step("verify")
                                .label("Second step")
                                .description("Verify email")
                                .content(|| text().content("Step 2 content: Verify email")),
                        ])
                        .completed(|| {
                            text().content("Completed, click back button to get to previous step")
                        })
                }),
            sapid::checkbox()
                .theme(theme.clone())
                .label("Vertical layout")
                .checked(vertical)
                .on_checked_change(move |next| vertical.revise(|v| *v = next)),
            sapid::group().theme(theme.clone()).fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Back")
                    .on_press(move || active.revise(|v| *v = v.saturating_sub(1))),
                sapid::button()
                    .theme(theme)
                    .label("Next step")
                    .on_press(move || active.revise(|v| *v = (*v + 1).min(2))),
            )),
        )))
        .show_in(scope)
    });
    crate::showcase_style_explorer::explorer("stepper-styles", Box::new(preview), PARTS)
}
