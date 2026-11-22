use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(40_f64);

        let direction = scope.cage(Some("ltr".to_owned()));
        let orientation = scope.cage(Some("up".to_owned()));
        let color = scope.cage("Blue".to_owned());
        let size = scope.cage(200_f64);
        let thickness = scope.cage(12_f64);
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                direction.get(),
                orientation.get(),
                color.get(),
                size.get().to_bits(),
                thickness.get().to_bits(),
            )]
        }))
        .key(Clone::clone)
        .children(move |_| {
            let theme = palette.clone();

            sapid::semi_circle_progress(value)
                .theme(theme.clone())
                .name("Monthly target")
                .size(size.get())
                .filled_segment_color(controls::accent(theme.clone(), &color.get()).accent)
                .fill_direction(if direction.get().as_deref() == Some("rtl") {
                    sapid::SemiCircleFillDirection::RightToLeft
                } else {
                    sapid::SemiCircleFillDirection::LeftToRight
                })
                .semi_orientation(if orientation.get().as_deref() == Some("down") {
                    sapid::SemiCircleOrientation::Down
                } else {
                    sapid::SemiCircleOrientation::Up
                })
                .thickness(thickness.get())
                .label("Label")
                .value_text(value.map(|v| format!("{v:.0} percent of the monthly target")))
                .style(Style {
                    font_size: Some(18.),
                    ..Style::new()
                })
        });
        view()
            .class("site-configurator site-display-configurator site-semi-circle-progress-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::option_segments(
                        "semi_circle_progress-direction",
                        "Fill direction",
                        direction,
                        &[("rtl", "Right to left"), ("ltr", "Left to right")],
                        &theme,
                    ),
                    controls::option_segments(
                        "semi_circle_progress-orientation",
                        "Orientation",
                        orientation,
                        &[("up", "Up"), ("down", "Down")],
                        &theme,
                    ),
                    controls::named_color_choices(
                        "semi-circle-fill-color",
                        "Filled segment color",
                        color.into(),
                        move |next| color.revise(|value| *value = next),
                        &theme,
                    ),
                    controls::range(
                        "semi_circle_progress-size",
                        "Size",
                        size,
                        120.,
                        450.,
                        1.,
                        &theme,
                    ),
                    controls::range(
                        "semi_circle_progress-thickness",
                        "Thickness",
                        thickness,
                        1.,
                        20.,
                        1.,
                        &theme,
                    ),
                    controls::range(
                        "semi_circle_progress-value",
                        "Value",
                        value,
                        0.,
                        100.,
                        1.,
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
