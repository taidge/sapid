use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    {
        let ink = theme.foreground;
        sapid::dialog("modal-close-icon")
            .theme(theme)
            .label("Open with custom close icon")
            .title("Account settings")
            .close_label("Return to settings")
            .close_content(move || {
                glory::ui::graphic()
                    .drawing(sapid::Icon::ArrowRight.drawing(ink))
                    .style(Style::new().width(Length::Px(18.)).height(Length::Px(18.)))
            })
            .description("Use the arrow in the header to return to the page.")
    }
}
