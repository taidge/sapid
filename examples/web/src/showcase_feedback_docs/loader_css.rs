use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    view()
        .class("sapid-demo-css-loader")
        .role(glory::render::Role::Status)
        .name("Loading")
        .style(Style {
            color: Some(theme.accent),
            ..Style::new().width(Length::Px(36.)).height(Length::Px(36.))
        })
}
