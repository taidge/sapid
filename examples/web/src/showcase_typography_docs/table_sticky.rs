use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::scroll_area("sticky-elements")
        .theme(theme.clone())
        .axes(glory::render::ScrollAxes::Vertical)
        .name("Sticky elements")
        .style(Style::new().height(Length::Px(200.)))
        .content(move |_| element_table(&theme, "periodic-sticky", true, false, true, false))
}
