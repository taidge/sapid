use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::alert("style-alert")
        .theme(theme.clone())
        .announcement(sapid::Announcement::Off)
        .title("Alert title")
        .content("Check the details before continuing.")
        .icon(
            glory::ui::graphic()
                .drawing(sapid::Icon::Info.drawing(theme.accent))
                .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
        )
        .closable(true);
    crate::showcase_style_explorer::explorer(
        "alert-styles",
        Box::new(preview),
        &[
            ("root", "Root element", ".sapid-alert"),
            (
                "wrapper",
                "Wrapper around `body` and `icon`",
                ".sapid-alert-wrapper",
            ),
            (
                "body",
                "Body element, contains `title` and `message`",
                ".sapid-alert-body",
            ),
            (
                "title",
                "Title element, contains `label` and `icon`",
                ".sapid-alert-title-row",
            ),
            ("label", "Title label", ".sapid-alert-title"),
            ("message", "Alert message", ".sapid-alert-content"),
            ("icon", "Icon element", ".sapid-alert-icon"),
            ("closeButton", "Close button", ".sapid-alert-close"),
        ],
    )
}
