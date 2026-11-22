use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::dropzone()
        .theme(theme.clone())
        .class("showcase-custom-zone")
        .label("Drop files to attach them")
        .style(
            Style::new()
                .min_height(Length::Px(120.))
                .width(Length::Percent(100.))
                .border_style(glory::style::BorderStyle::Dashed)
                .border_color(theme.accent)
                .border_width(Edges::all(2.))
                .radius(10.),
        )
}
