use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .class("site-reference-actions site-ref-input-icon")
        .gap(16.)
        .fill((
            sapid::text_field()
                .theme(theme.clone())
                .size(sapid::Size::Small)
                .name("Small input")
                .placeholder("sm size input")
                .style(Style::new().width(Length::Px(185.)).height(Length::Px(36.))),
            sapid::action_icon("ActionIcon the same size as inputs")
                .theme(theme)
                .variant(sapid::ActionIconVariant::Default)
                .style(Style::new().width(Length::Px(36.)).height(Length::Px(36.)))
                .icon(text().content("SM")),
        ))
}
