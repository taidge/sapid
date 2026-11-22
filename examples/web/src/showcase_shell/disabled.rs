use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let off = scope.cage(false);
        let note = scope.cage(String::new());
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![off.get()]))
            .key(Clone::clone)
            .children(move |row| {
                let shell = sapid::app_shell("disable-regions")
                    .theme(palette.clone())
                    .style(Style::new().height(Length::Px(250.)))
                    .breakpoint(420.);
                let shell = if row.value.get() {
                    shell
                } else {
                    shell
                        .header(|c| sapid::text().theme(c.theme).content("Header"))
                        .sidebar(
                            sapid::sidebar("disabled-navigation")
                                .width(120.)
                                .rail_width(40.)
                                .content(|c| sapid::text().theme(c.theme).content("Navigation")),
                        )
                };
                shell.main(move |c| {
                    sapid::text_field()
                        .theme(c.theme)
                        .name("Document note")
                        .placeholder("Document note")
                        .value(note)
                        .on_value_change(move |next| note.revise(|v| *v = next))
                })
            });
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Toggle disabled")
                    .pressed(off)
                    .on_press(move || off.revise(|v| *v = !*v)),
                preview,
            ))
            .show_in(scope)
    })
}
