use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::app_shell("outer-workspace")
        .theme(theme)
        .style(Style::new().height(Length::Px(350.)))
        .breakpoint(420.)
        .header(|c| sapid::text().theme(c.theme).content("Application header"))
        .sidebar(
            sapid::sidebar("outer-navigation")
                .width(110.)
                .rail_width(40.)
                .content(|c| sapid::text().theme(c.theme).content("Workspaces")),
        )
        .main(|c| {
            sapid::app_shell("inner-project")
                .theme(c.theme)
                .style(Style::new().height(Length::Px(230.)))
                .breakpoint(320.)
                .header(|c| sapid::text().theme(c.theme).content("Project header"))
                .sidebar(
                    sapid::sidebar("inner-navigation")
                        .width(88.)
                        .rail_width(36.)
                        .content(|c| sapid::text().theme(c.theme).content("Folders")),
                )
                .main(|c| {
                    sapid::text_field()
                        .theme(c.theme)
                        .name("Nested project note")
                        .placeholder("Project note")
                })
        })
}
