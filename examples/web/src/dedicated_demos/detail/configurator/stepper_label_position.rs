use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(1usize);
        sapid::stepper("reference-site-step-labels-below")
            .theme(theme.clone())
            .class("site-step-labels-below")
            .name("Label position")
            .active(active)
            .selectable(true)
            .on_step_change(move |next| active.revise(|v| *v = next))
            .steps(
                [
                    ("1", "Account", ""),
                    ("2", "Verification", ""),
                    ("3", "Access", ""),
                ]
                .map(|(key, label, description)| {
                    sapid::step(key).label(label).description(description)
                }),
            )
            .show_in(scope)
    })
}
