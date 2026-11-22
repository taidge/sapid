use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage("Blue".to_owned());
        let align = scope.cage(Some("start".to_owned()));
        let reverse = scope.cage(false);
        let opposite = scope.cage(false);
        let active = scope.cage(1.);
        let bullet = scope.cage(25.);
        let line = scope.cage(4.);
        let radius = scope.cage(32.);
        let palette = theme.clone();
        const EVENTS: [(&str, &str, &str, &str); 4] = [
            (
                "branch",
                "New branch",
                "You've created new branch fix-notifications from master",
                "2 hours ago",
            ),
            (
                "commit",
                "Commits",
                "You've pushed 23 commits to fix-notifications branch",
                "52 minutes ago",
            ),
            (
                "request",
                "Pull request",
                "You've submitted a pull request Fix incorrect notification message (#187)",
                "34 minutes ago",
            ),
            (
                "review",
                "Code review",
                "Robert Gluesticker left a code review on your pull request",
                "12 minutes ago",
            ),
        ];
        let progress = active.map(|n| {
            if *n < 0. {
                sapid::TimelineProgress::Pending
            } else {
                sapid::TimelineProgress::Active(EVENTS[*n as usize].0.to_owned())
            }
        });
        let config = scope.bond(move || {
            vec![(
                color.get(),
                align.get(),
                reverse.get(),
                opposite.get(),
                bullet.get() as u8,
                line.get() as u8,
                radius.get() as u8,
            )]
        });
        let preview = each(config).key(Clone::clone).children(move |_| {
            let theme = controls::accent(palette.clone(), &color.get());
            let mut list = sapid::timeline("map-history")
                .theme(theme.clone())
                .name("Repository activity")
                .progress(progress)
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .max_width(Limit::Px(320.)),
                )
                .reverse_active(reverse.get())
                .bullet_size(bullet.get())
                .line_width(line.get())
                .bullet_radius(radius.get())
                .align(match align.get().as_deref() {
                    Some("end") => sapid::TimelineAlign::End,
                    Some("alternate") => sapid::TimelineAlign::Alternate,
                    _ => sapid::TimelineAlign::Start,
                });
            for (i, (key, title, description, time)) in EVENTS.into_iter().enumerate() {
                let mut item = sapid::timeline_item(key)
                    .title(title)
                    .description(description);
                if i == 2 {
                    item = item.line_style(glory::style::BorderStyle::Dashed);
                }
                if opposite.get() {
                    item = item.opposite(
                        sapid::text()
                            .theme(theme.clone())
                            .variant(sapid::TextVariant::Small)
                            .content(time),
                    );
                } else {
                    item = item.timestamp(time);
                }
                list = list.item(item);
            }
            list
        });
        view()
            .class("site-configurator site-display-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::color_choices(color, &theme),
                    controls::range("timeline-active", "Active", active, -1., 3., 1., &theme),
                    controls::switch("Reverse active", reverse, &theme),
                    controls::range("timeline-line", "Line width", line, 1., 6., 1., &theme),
                    controls::range(
                        "timeline-bullet",
                        "Bullet size",
                        bullet,
                        18.,
                        40.,
                        1.,
                        &theme,
                    ),
                    controls::pixel_scale(
                        "timeline-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                    controls::select(
                        "timeline-align",
                        "Align",
                        align,
                        &[
                            ("start", "Start"),
                            ("end", "End"),
                            ("alternate", "Alternate"),
                        ],
                        &theme,
                    ),
                    controls::switch("Opposite timestamps", opposite, &theme),
                ],
            ))
            .show_in(scope)
    })
}
