use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::notification("style-reconnect")
            .theme(theme.clone())
            .title("Please wait")
            .content("The application is trying to reconnect to the server.")
            .loading(true)
            .closable(false),
        sapid::notification("style-notice")
            .theme(theme.clone())
            .title("Changes saved")
            .content("Your new settings are available.")
            .icon(
                glory::ui::graphic()
                    .drawing(sapid::Icon::Check.drawing(theme.accent))
                    .style(Style::new().width(Length::Px(18.)).height(Length::Px(18.))),
            ),
    ));
    crate::showcase_style_explorer::explorer(
        "notification-styles",
        Box::new(preview),
        &[
            ("root", "Root element", ".sapid-notification"),
            (
                "loader",
                "Loader component, displayed only when `loading` prop is set",
                ".sapid-notification-loader",
            ),
            (
                "icon",
                "Icon component, displayed only when `icon` prop is set",
                ".sapid-alert-icon:not(:has(.sapid-notification-loader)), .sapid-notification-icon-content:not([hidden])",
            ),
            (
                "body",
                "Notification body, contains all other elements",
                ".sapid-alert-body",
            ),
            (
                "title",
                "Title element, displayed only when `title` prop is set",
                ".sapid-alert-title",
            ),
            (
                "description",
                "Description displayed below the title",
                ".sapid-alert-content",
            ),
            ("closeButton", "Close button element", ".sapid-alert-close"),
        ],
    )
}
