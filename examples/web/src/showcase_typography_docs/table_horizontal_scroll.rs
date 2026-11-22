use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::scroll_area("elements-horizontal")
        .theme(theme.clone())
        .axes(glory::render::ScrollAxes::Horizontal)
        .name("Periodic table")
        .content(move |_| {
            element_table(&theme, "periodic-horizontal", false, false, false, false)
                .style(Style::new().min_width(Length::Px(500.)))
        })
}
