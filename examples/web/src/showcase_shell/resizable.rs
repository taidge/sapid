use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(true);
        let width = scope.cage(32.);
        sapid::app_shell("resizable-workspace")
            .theme(theme.clone())
            .header(move |ctx| {
                sapid::button()
                    .theme(ctx.theme)
                    .label(open.map(|open| {
                        if *open {
                            "Collapse folders"
                        } else {
                            "Restore folders"
                        }
                        .to_owned()
                    }))
                    .pressed(open)
                    .on_press(move || open.revise(|open| *open = !*open))
            })
            .main(move |ctx| {
                let theme = ctx.theme;
                let folder_theme = theme.clone();
                sapid::splitter("workspace-folders")
                    .theme(theme.clone())
                    .name("Folder pane width")
                    .enabled(open)
                    .value(width.map(move |width| if open.get() { *width } else { 0. }))
                    .min(open.map(|open| if *open { 16. } else { 0. }))
                    .max(70.)
                    .step(4.)
                    .on_value_change(move |next| width.revise(|width| *width = next))
                    .style(Style::new().height(Length::Px(180.)))
                    .first(move || {
                        let theme = folder_theme.clone();
                        when(open, move || {
                            sapid::stack().theme(theme.clone()).gap(6.).fill((
                                text().content("Folders"),
                                text().content("Inbox"),
                                text().content("Archive"),
                            ))
                        })
                    })
                    .second(move || {
                        sapid::text_field()
                            .theme(theme.clone())
                            .name("Workspace note")
                            .placeholder("Write a note")
                            .style(Style::new().width(Length::Percent(100.)))
                    })
            })
            .show_in(scope)
    })
}
