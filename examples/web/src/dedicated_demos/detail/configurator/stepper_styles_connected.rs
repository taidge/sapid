use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(1usize);
        sapid::stepper("site-ref-step-connected")
            .theme(theme.clone())
            .class("site-ref-step-connected")
            .name("Connected controls")
            .active(active)
            .selectable(true)
            .allow_skip(true)
            .on_step_change(move |next| active.revise(|v| *v = next))
            .steps(
                [
                    ("one", "Step 1", "Create an account"),
                    ("two", "Step 2", "Verify email"),
                    ("three", "Step 3", "Get full access"),
                ]
                .map(|(key, label, description)| {
                    sapid::step(key).label(label).description(description)
                }),
            )
            .show_in(scope)
    })
}
