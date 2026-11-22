use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::scroll_area("elements-height")
        .theme(theme.clone())
        .axes(glory::render::ScrollAxes::Both)
        .name("Elements by atomic number")
        .style(Style::new().height(Length::Px(200.)))
        .content(move |_| {
            element_table(&theme, "periodic-height", true, false, false, false)
                .style(Style::new().min_width(Length::Px(500.)))
        })
}
