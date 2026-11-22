use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    const EVENTS: [(&str, &str, &str, &str, sapid::Icon); 4] = [
        (
            "branch",
            "Branch created",
            "Mira started a branch for the offline map.",
            "2 hours ago",
            sapid::Icon::ChevronRight,
        ),
        (
            "commit",
            "Changes committed",
            "The new map tiles and route legend are ready.",
            "52 minutes ago",
            sapid::Icon::Circle,
        ),
        (
            "request",
            "Pull request opened",
            "The route is ready for someone else to try.",
            "34 minutes ago",
            sapid::Icon::ArrowRight,
        ),
        (
            "review",
            "Review received",
            "Alex checked the trail labels on a smaller screen.",
            "12 minutes ago",
            sapid::Icon::Check,
        ),
    ];
    sapid::timeline("release-history")
        .theme(theme.clone())
        .name("Map release history")
        .active("commit")
        .items(EVENTS.map(|(key, title, description, time, icon)| {
            sapid::timeline_item(key)
                .title(title)
                .description(description)
                .timestamp(time)
                .marker(
                    glory::ui::graphic()
                        .drawing(icon.drawing(theme.accent_text))
                        .style(Style::new().width(Length::Px(14.)).height(Length::Px(14.))),
                )
        }))
}
