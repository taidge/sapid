use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(1usize);
        sapid::stepper("reference-site-step-icons-right")
            .theme(theme.clone())
            .class("site-step-icons-right")
            .name("Icon position")
            .active(active)
            .selectable(true)
            .on_step_change(move |next| active.revise(|v| *v = next))
            .steps(
                [
                    ("1", "Step 1", "Create an account"),
                    ("2", "Step 2", "Verify email"),
                    ("3", "Step 3", "Get full access"),
                ]
                .map(|(key, label, description)| {
                    sapid::step(key).label(label).description(description)
                }),
            )
            .show_in(scope)
    })
}
