use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::anchor()
        .theme(theme.clone())
        .href("https://glory.rs")
        .label("Back to project website")
        .open_in(glory::render::LinkOpen::NewContext)
        .class("site-ref-inline-back")
        .fill(
            group().fill((
                glory::ui::graphic()
                    .drawing(sapid::Icon::ArrowLeft.drawing(theme.accent))
                    .style(Style::new().width(Length::Px(12.)).height(Length::Px(12.))),
                text().content("Back to project website"),
            )),
        )
}
