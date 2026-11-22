use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(widget(move |scope| {
            let open = scope.cage(false);
            let picked = scope.cage("Select framework".to_owned());
            let search = scope.cage(String::new());
            let inside = theme.clone();
            let picker = sapid::popover("popover-search")
                .theme(theme.clone())
                .width(sapid::PopoverWidth::Px(250.))
                .show_close(false)
                .label(picked)
                .open(open)
                .on_open_change(move |next| open.revise(|v| *v = next))
                .content(move || {
                    sapid::combobox("popover-search-choices")
                        .theme(inside.clone())
                        .name("Search frameworks")
                        .placeholder("Search...")
                        .options(
                            ["React", "Angular", "Vue", "Svelte"]
                                .map(|label| sapid::select_option(label, label)),
                        )
                        .inline_list(true)
                        .open(true)
                        .search(search)
                        .on_search_change(move |next| search.revise(|v| *v = next))
                        .empty_message("Nothing found...")
                        .on_value_change(move |next| {
                            if let Some(next) = next {
                                picked.revise(|v| *v = next);
                                open.revise(|v| *v = false);
                            }
                        })
                });
            picker.show_in(scope)
        }))
}
