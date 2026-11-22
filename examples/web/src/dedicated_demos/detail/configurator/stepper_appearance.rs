use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(1usize);
        let palette = theme.clone();
        let color = scope.cage("Blue".to_owned());
        let size = scope.cage(Some("md".to_owned()));
        let radius = scope.cage(32.);
        let preview = each(scope.bond(move || vec![(color.get(), size.get(), radius.get() as u8)]))
            .key(Clone::clone)
            .children(move |row| {
                let (color, size, radius) = row.value.get();
                sapid::stepper("step-appearance")
                    .theme(controls::accent(palette.clone(), &color))
                    .size(controls::size_value(size.as_deref().unwrap_or("md")))
                    .icon_radius(f64::from(radius))
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
                vec![
                    controls::color_choices(color, &theme),
                    controls::pixel_scale(
                        "step-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                    controls::size_slider(size, &theme),
                ],
            ))
            .show_in(scope)
    })
}
