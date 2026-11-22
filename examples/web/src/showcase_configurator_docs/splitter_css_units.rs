use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let pixels = scope.cage(240f64);
        let width = scope.cage(860f64);
        let val = scope.bond(move || (pixels.get().max(160.) / width.get() * 100.).min(50.));
        let can_resize = width.map(|v| *v > 320.);
        let min = width.map(|v| if *v > 320. { 160. / v * 100. } else { 0. });
        view()
            .class("site-ref-css-splitter")
            .on(glory::render::EventKind::Resize, move |event| {
                if let glory::render::Event::Resize { size, .. } = event
                    && size.width > 0.
                {
                    width.revise(|v| *v = size.width);
                }
            })
            .fill(
                sapid::splitter("sidebar-pixel-units")
                    .theme(theme.clone())
                    .name("Sidebar width")
                    .class("site-splitter-colors")
                    .enabled(can_resize)
                    .with_grip(true)
                    .grip_size(52., 10.)
                    .value(val)
                    .min(min)
                    .max(50.)
                    .on_value_change(move |next| pixels.revise(|v| *v = width.get() * next / 100.))
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .height(Length::Px(200.)),
                    )
                    .first(|| {
                        view()
                            .class("site-splitter-first")
                            .fill(text().content("Fixed 240px sidebar"))
                    })
                    .second(|| {
                        view()
                            .class("site-splitter-second")
                            .fill(text().content("Flexible content"))
                    }),
            )
            .show_in(scope)
    })
}
