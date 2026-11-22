use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::marquee()
        .visual_content(|| logo_strip(true, true))
        .theme(theme)
        .text("Sapid logos in eight colors")
        .direction(sapid::MarqueeDirection::Up)
        .style(
            Style::new()
                .width(Length::Px(110.))
                .max_width(Limit::Percent(100.))
                .height(Length::Px(280.)),
        )
        .class("sapid-marquee-fade-vertical sapid-demo-centered-marquee")
}
