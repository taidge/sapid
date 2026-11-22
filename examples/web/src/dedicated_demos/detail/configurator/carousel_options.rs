use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let current = scope.cage(0usize);
        let orientation = scope.cage("horizontal".to_owned());
        let gap = scope.cage(2.);
        let offset = scope.cage(1.);
        let size = scope.cage(26.);
        let controls = scope.cage(true);
        let indicators = scope.cage(false);
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                orientation.get(),
                gap.get() as usize,
                offset.get() as usize,
                size.get() as u8,
                controls.get(),
                indicators.get(),
            )]
        }))
        .key(Clone::clone)
        .children(move |entry| {
            let (axis, gap, offset, size, controls, indicators) = entry.value.get();
            let blue = palette.accent;
            sapid::carousel()
                .name("Options slides")
                .theme(palette.clone())
                .count(5usize)
                .viewport_height(200.)
                .slide_fraction(0.7)
                .overscan(2)
                .orientation(if axis == "vertical" {
                    glory::render::Orientation::Vertical
                } else {
                    glory::render::Orientation::Horizontal
                })
                .slide_gap([10., 12., 16., 20., 32.][gap])
                .controls_offset([10., 12., 16., 20., 32.][offset])
                .control_size(f64::from(size))
                .with_controls(controls)
                .with_indicators(indicators)
                .slide_align(Align::Start)
                .current(current)
                .on_current_change(move |next| current.revise(|old| *old = next))
                .render_slide(move |index| {
                    view()
                        .class("site-carousel-number")
                        .style(Style {
                            background: Some(blue),
                            color: Some(glory::style::Color(255, 255, 255, 255)),
                            ..Style::new()
                        })
                        .fill(text().content((index + 1).to_string()))
                })
        });
        view()
            .class("site-configurator site-carousel-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::segmented_choice(
                        "carousel-orientation",
                        "Orientation",
                        orientation,
                        &[("horizontal", "Horizontal"), ("vertical", "Vertical")],
                        &theme,
                    ),
                    controls::field_scale("carousel-gap".into(), "Slide gap", gap, &theme),
                    controls::field_scale(
                        "carousel-offset".into(),
                        "Controls offset",
                        offset,
                        &theme,
                    ),
                    controls::range("carousel-size", "Control size", size, 14., 40., 1., &theme),
                    controls::switch("With controls", controls, &theme),
                    controls::switch("With indicators", indicators, &theme),
                ],
            ))
            .show_in(scope)
    })
}
