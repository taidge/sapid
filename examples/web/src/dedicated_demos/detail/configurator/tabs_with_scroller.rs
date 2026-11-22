use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let area_theme = theme.clone();
    sapid::scroller("reference-tab-scroller")
        .area(move |area| {
            area.theme(area_theme.clone()).name("Scroll tabs").style(
                Style::new()
                    .height(Length::Px(58.))
                    .max_width(Limit::Px(460.)),
            )
        })
        .content(move |_| {
            let mut tabs = sapid::tabs("reference-scrolled-tabs")
                .theme(theme.clone())
                .name("Scrollable tab list")
                .default_value("tab-1")
                .class("site-reference-scrolled-tabs");
            for (i, label) in [
                "First tab",
                "Second tab",
                "Third tab",
                "Fourth tab",
                "Fifth tab",
                "Sixth tab",
                "Seventh tab",
                "Eighth tab",
                "Ninth tab",
                "Tenth tab",
            ]
            .into_iter()
            .enumerate()
            {
                tabs = tabs.tab(
                    sapid::tab(format!("tab-{}", i + 1))
                        .label(label)
                        .content(group),
                );
            }
            tabs
        })
}
