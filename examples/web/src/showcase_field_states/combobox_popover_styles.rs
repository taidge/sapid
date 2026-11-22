use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = widget(move |scope| {
        let open = scope.cage(true);
        let picked = scope.cage("Select technology".to_owned());
        let list_theme = theme.clone();
        sapid::popover("styles-popover")
            .theme(theme.clone())
            .open(open)
            .on_open_change(move |next| open.revise(|value| *value = next))
            .label(picked)
            .show_close(false)
            .width(sapid::PopoverWidth::Px(200.))
            .content(move || {
                sapid::combobox("styles-popover-search")
                    .theme(list_theme.clone())
                    .name("Search technologies")
                    .placeholder("Search...")
                    .open(true)
                    .inline_list(true)
                    .empty_message("Nothing found...")
                    .group(
                        sapid::select_group("Frontend")
                            .options(["React", "Angular"].map(|v| sapid::select_option(v, v))),
                    )
                    .group(sapid::select_group("Backend").options([
                        sapid::select_option("Node", "Node.js"),
                        sapid::select_option("Django", "Django"),
                    ]))
                    .on_value_change(move |choice| {
                        if let Some(next) = choice {
                            picked.revise(|value| {
                                *value = if next == "Node" {
                                    "Node.js".into()
                                } else {
                                    next
                                }
                            });
                            open.revise(|value| *value = false);
                        }
                    })
            })
            .show_in(scope)
    });
    crate::showcase_style_explorer::explorer(
        "combobox-popover-styles",
        Box::new(
            sapid::stack()
                .class("site-field-state-example")
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .max_width(Limit::Px(340.)),
                )
                .fill(preview),
        ),
        &[
            ("dropdown", "Dropdown root element", "[role=\"listbox\"]"),
            ("options", "Options wrapper", "[role=\"listbox\"]"),
            ("option", "Option", "[role=\"option\"]"),
            ("empty", "Nothing found message", ".sapid-combobox-empty"),
            ("group", "Options group wrapper", "[role=\"group\"]"),
            (
                "groupLabel",
                "Options group label",
                "[role=\"group\"] > [data-gly-kind=\"text\"]",
            ),
            (
                "search",
                "Search input, displayed only when `searchable` prop is set",
                "input.sapid-combobox",
            ),
        ],
    )
}
