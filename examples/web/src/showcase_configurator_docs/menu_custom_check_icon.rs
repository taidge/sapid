use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let rows = [
            ("open", "Open", true),
            ("drafts", "Drafts", false),
            ("archived", "Archived", false),
        ]
        .map(|(key, label, initial)| {
            let checked = scope.cage(initial);
            sapid::menu_checkbox(key)
                .label(label)
                .checked(checked)
                .on_checked_change(move |v| checked.revise(|x| *x = v))
                .close_on_select(false)
        });
        sapid::dropdown_menu("reference-menu-check-icon")
            .theme(theme.clone())
            .label("Filters")
            .width(220.)
            .class("site-menu-check-reference")
            .items(rows)
            .show_in(scope)
    })
}
