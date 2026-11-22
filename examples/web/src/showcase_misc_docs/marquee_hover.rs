use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let inside = scope.cage(false);
        sapid::marquee()
            .visual_content(|| logo_strip(false, false))
            .theme(theme.clone())
            .text("Sapid logos in eight colors")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Percent(100.)),
            )
            .paused(inside)
            .on(glory::render::EventKind::PointerWithin, move |e| {
                if let glory::render::Event::PointerWithin { phase, .. } = e {
                    inside.revise(|v| *v = phase != glory::render::PointerPhase::Leave);
                }
            })
            .show_in(scope)
    })
}
