use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(1usize);
        let palette = theme.clone();
        let diameter = scope.cage(42.);
        let preview = each(scope.bond(move || vec![diameter.get() as u8]))
            .key(Clone::clone)
            .children(move |row| {
                sapid::stepper("step-icon-size")
                    .theme(palette.clone())
                    .icon_size(f64::from(row.value.get()))
                    .name("Configured steps")
                    .class("site-two-step-example")
                    .active(active)
                    .selectable(true)
                    .on_step_change(move |n| active.revise(|v| *v = n))
                    .steps(
                        [
                            ("one", "Step 1", "Create an account"),
                            ("two", "Step 2", "Verify email"),
                        ]
                        .map(|(key, label, description)| {
                            sapid::step(key).label(label).description(description)
                        }),
                    )
            });
        view()
            .class("site-configurator site-layout-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::range(
                    "step-icon",
                    "Icon size",
                    diameter,
                    32.,
                    62.,
                    5.,
                    &theme,
                )],
            ))
            .show_in(scope)
    })
}
