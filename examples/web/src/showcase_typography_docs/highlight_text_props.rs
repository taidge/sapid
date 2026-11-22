use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    view()
        .class("sapid-demo-highlight-text-props")
        .style(Style {
            ..Style::new().width(Length::Percent(100.)).font_weight(700.)
        })
        .fill(
            sapid::highlight()
                .theme(theme)
                .content("Sapid documentation")
                .query("Sapid"),
        )
}
