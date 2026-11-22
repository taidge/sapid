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
            let many = scope.cage(Vec::<String>::new());
            let inside = theme.clone();
            let picker = sapid::popover("popover-multiple")
                .theme(theme.clone())
                .width(sapid::PopoverWidth::Px(250.))
                .show_close(false)
                .label(many.map(|v| {
                    if v.is_empty() {
                        "Select frameworks".to_owned()
                    } else {
                        v.join(", ")
                    }
                }))
                .open(open)
                .on_open_change(move |next| open.revise(|v| *v = next))
                .content(move || {
                    sapid::multi_select("popover-multiple-choices")
                        .theme(inside.clone())
                        .name("Search frameworks")
                        .placeholder("Search...")
                        .options(
                            ["React", "Angular", "Vue", "Svelte"]
                                .map(|label| sapid::select_option(label, label)),
                        )
                        .inline_list(true)
                        .open(true)
                        .value(many)
                        .on_value_change(move |next| many.revise(|value| *value = next))
                });
            picker.show_in(scope)
        }))
}
