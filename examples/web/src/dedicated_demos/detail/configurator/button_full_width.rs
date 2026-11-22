use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::button()
        .theme(theme)
        .label("Full width button")
        .style(Style::new().width(Length::Percent(100.)))
}
