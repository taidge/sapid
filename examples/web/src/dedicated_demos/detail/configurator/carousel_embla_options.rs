use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let current = scope.cage(0usize);
        let align = scope.cage("center".to_owned());
        let looping = scope.cage(true);
        let free = scope.cage(false);
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(align.get(), looping.get(), free.get())]))
            .key(Clone::clone)
            .children(move |entry| {
                let (align, looping, free) = entry.value.get();
                let blue = palette.accent;
                sapid::carousel()
                    .name("Embla options slides")
                    .theme(palette.clone())
                    .count(5usize)
                    .viewport_height(200.)
                    .slide_fraction(0.7)
                    .overscan(2)
                    .slide_gap(16.)
                    .control_size(26.)
                    .controls_offset(12.)
                    .with_indicators(true)
                    .slide_align(match align.as_str() {
                        "start" => Align::Start,
                        "end" => Align::End,
                        _ => Align::Center,
                    })
                    .looping(looping)
                    .drag_free(free)
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
                        "carousel-align",
                        "Align",
                        align,
                        &[("start", "Start"), ("center", "Center"), ("end", "End")],
                        &theme,
                    ),
                    controls::switch("Loop", looping, &theme),
                    controls::switch("Drag free", free, &theme),
                ],
            ))
            .show_in(scope)
    })
}
