use glory::prelude::*;

const PARTS: &[(&str, &str, &str)] = &[
    ("root", "Root element (`Tabs` component)", ".sapid-tabs"),
    (
        "list",
        "List of tabs (`Tabs.List` component)",
        ".sapid-tab-list",
    ),
    (
        "panel",
        "Panel with tab content (`Tabs.Panel` component)",
        ".sapid-tab-panel",
    ),
    ("tab", "Tab button (`Tabs.Tab` component)", ".sapid-tab"),
    ("tabLabel", "Label of `Tabs.Tab`", ".site-tabs-label"),
    (
        "tabSection",
        "Left and right sections of `Tabs.Tab`",
        ".site-tabs-section",
    ),
];

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = widget(move |scope| {
        let theme = theme.clone();
        let mut tabs = sapid::tabs("styles-tabs")
            .theme(theme.clone())
            .name("Tab styles")
            .default_value("gallery");
        for (key, label, icon) in [
            ("gallery", "Gallery", sapid::Icon::Calendar),
            ("messages", "Messages", sapid::Icon::Info),
            ("settings", "Settings", sapid::Icon::Circle),
        ] {
            let ink = theme.foreground;
            tabs = tabs.tab(
                sapid::tab(key)
                    .label(label)
                    .trigger_content(move || {
                        let graphic = glory::ui::graphic()
                            .class("site-tabs-section")
                            .drawing(icon.drawing(ink))
                            .style(Style::new().width(Length::Px(12.)).height(Length::Px(12.)));
                        let name = text().class("site-tabs-label").content(label);
                        if key == "settings" {
                            group().fill((name, graphic))
                        } else {
                            group().fill((graphic, name))
                        }
                    })
                    .content(move || text().content(format!("{label} tab content"))),
            );
        }
        (tabs).show_in(scope)
    });
    crate::showcase_style_explorer::explorer("tabs-styles", Box::new(preview), PARTS)
}
