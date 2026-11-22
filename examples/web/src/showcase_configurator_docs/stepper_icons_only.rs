use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let current = scope.cage(2usize);
        sapid::stepper("icons-only")
            .theme(theme.clone())
            .class("site-ref-icon-step site-ref-error-step")
            .name("Registration progress")
            .active(current)
            .allow_skip(true)
            .selectable(true)
            .on_step_change(move |n| current.revise(|v| *v = n))
            .steps(
                [
                    ("account", "Step 1", "Create an account", sapid::Icon::Check),
                    ("verify", "Step 2", "Verify email", sapid::Icon::Info),
                    ("secure", "Step 3", "Get full access", sapid::Icon::Circle),
                ]
                .map(|(key, label, description, _icon)| {
                    sapid::step(key)
                        .label(label)
                        .description(description)
                        .status(if key == "verify" {
                            Some(sapid::StepStatus::Error)
                        } else {
                            None
                        })
                }),
            )
            .show_in(scope)
    })
}
