use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let events = [
        (
            "branch",
            "New branch",
            "Created a branch for the redesign",
            "2 hours ago",
        ),
        (
            "commit",
            "Commits",
            "Added the first round of changes",
            "52 minutes ago",
        ),
        (
            "request",
            "Pull request",
            "Opened the changes for review",
            "34 minutes ago",
        ),
    ];
    sapid::timeline("timeline-start-true")
        .theme(theme.clone())
        .name("Activity")
        .align(sapid::TimelineAlign::Start)
        .active("commit")
        .items(events.map(|(key, title, description, time)| {
            let item = sapid::timeline_item(key)
                .title(title)
                .description(description);
            item.opposite(sapid::text().theme(theme.clone()).content(time))
        }))
}
