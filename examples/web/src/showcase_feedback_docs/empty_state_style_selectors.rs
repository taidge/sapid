use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::empty_state()
        .theme(theme.clone())
        .illustration(
            glory::ui::graphic()
                .drawing(sapid::Icon::Search.drawing(theme.muted))
                .style(Style::new().width(Length::Px(36.)).height(Length::Px(36.))),
        )
        .title("No results found")
        .description("Try adjusting your filters or searching with different keywords.")
        .actions(sapid::button().theme(theme).label("Reset filters"));
    crate::showcase_style_explorer::explorer(
        "empty-state-styles",
        Box::new(preview),
        &[
            ("root", "Root element", ".sapid-empty-state"),
            (
                "body",
                "Wrapper around `title`, `description` and `actions`",
                ".sapid-empty-body-group",
            ),
            (
                "indicator",
                "Wrapper around the icon or illustration",
                ".sapid-empty-illustration",
            ),
            ("title", "Title element", ".sapid-empty-title"),
            (
                "description",
                "Description element",
                ".sapid-empty-description",
            ),
            (
                "actions",
                "Wrapper around action buttons",
                ".sapid-empty-actions",
            ),
        ],
    )
}
