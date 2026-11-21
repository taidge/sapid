use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage("Blue".to_owned());
        let align = scope.cage(Some("start".to_owned()));
        let reverse = scope.cage(false);
        let opposite = scope.cage(false);
        let active = scope.cage(1.);
        let bullet = scope.cage(28.);
        let line = scope.cage(2.);
        let radius = scope.cage(12.);
        let palette = theme.clone();
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
        let progress =
            active.map(|n| sapid::TimelineProgress::Active(EVENTS[*n as usize].0.to_owned()));
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
                .name("Map release history")
                .progress(progress)
                .style(Style::new().width(Length::Percent(100.)))
                .reverse_active(reverse.get())
                .bullet_size(bullet.get())
                .line_width(line.get())
                .bullet_radius(radius.get())
                .align(match align.get().as_deref() {
                    Some("end") => sapid::TimelineAlign::End,
                    Some("alternate") => sapid::TimelineAlign::Alternate,
                    _ => sapid::TimelineAlign::Start,
                });
            for (i, (key, title, description, time, icon)) in EVENTS.into_iter().enumerate() {
                let ink = theme.clone();
                let reversed = reverse.get();
                let marker = glory::ui::graphic()
                    .drawing(active.map(move |n| {
                        let done = if reversed {
                            i > *n as usize
                        } else {
                            i < *n as usize
                        };
                        icon.drawing(if done {
                            ink.contrasting_foreground(ink.accent, ink.on_accent)
                        } else {
                            ink.foreground
                        })
                    }))
                    .style(
                        Style::new()
                            .width(Length::Px(bullet.get() * 0.54))
                            .height(Length::Px(bullet.get() * 0.54)),
                    );
                let mut item = sapid::timeline_item(key)
                    .title(title)
                    .description(description)
                    .marker(marker);
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
            .class("site-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::color_choices(color, &theme),
                    controls::range(
                        "timeline-active",
                        "Active event",
                        active,
                        0.,
                        3.,
                        1.,
                        &theme,
                    ),
                    controls::switch("Reverse active", reverse, &theme),
                    controls::range("timeline-line", "Line width", line, 0., 8., 1., &theme),
                    controls::range(
                        "timeline-bullet",
                        "Bullet size",
                        bullet,
                        16.,
                        48.,
                        2.,
                        &theme,
                    ),
                    controls::radius_slider(radius, &theme),
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
