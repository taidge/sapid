use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::scroller("scrollable-tabs")
        .area(|area| {
            area.theme(theme.clone())
                .name("Scrollable project tabs")
                .style(
                    Style::new()
                        .height(Length::Px(130.))
                        .max_width(Limit::Px(360.)),
                )
        })
        .content(move |_| {
            let mut tabs = sapid::tabs("project-section-tabs")
                .theme(theme.clone())
                .default_value("Activity")
                .style(Style::new().min_width(Length::Px(740.)));
            for title in [
                "Activity",
                "Documents",
                "Members",
                "Messages",
                "Changes",
                "Archive",
            ] {
                tabs = tabs.tab(
                    sapid::tab(title)
                        .label(title)
                        .content(move || text().content(format!("{title} panel"))),
                );
            }
            tabs
        })
}
