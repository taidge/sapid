use glory::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Options {
    Frameworks,
    Scrollable,
    Limited,
}

pub(super) fn picker(
    theme: sapid::Theme,
    key: &'static str,
    picked: Cage<Option<String>>,
    empty_label: &'static str,
    allow_deselect: bool,
    options: Options,
    form_name: Option<&'static str>,
) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let search = scope.cage(String::new());
        let inside = theme.clone();
        sapid::popover(key)
            .theme(theme.clone())
            .width(sapid::PopoverWidth::Px(220.))
            .trigger_style(Style::new().width(Length::Px(200.)))
            .show_close(false)
            .label(picked.map(move |value| {
                value
                    .as_ref()
                    .map(|key| {
                        key.strip_prefix("option-")
                            .map_or_else(|| key.clone(), |index| format!("Option {index}"))
                    })
                    .unwrap_or_else(|| empty_label.to_owned())
            }))
            .open(open)
            .on_open_change(move |next| {
                search.revise(|query| query.clear());
                open.revise(|value| *value = next);
            })
            .content_policy(sapid::ContentPolicy::KeepMounted)
            .content(move || {
                let choices = match options {
                    Options::Frameworks => ["React", "Angular", "Vue", "Svelte"]
                        .map(|label| sapid::select_option(label, label))
                        .into_iter()
                        .collect::<Vec<_>>(),
                    Options::Scrollable | Options::Limited => {
                        let count = if options == Options::Limited {
                            1000
                        } else {
                            50
                        };
                        (0..count)
                            .map(|index| {
                                sapid::select_option(
                                    format!("option-{index}"),
                                    format!("Option {index}"),
                                )
                            })
                            .collect()
                    }
                };
                let mut picker = sapid::combobox(format!("{key}-options"))
                    .theme(inside.clone())
                    .name("Choose an option")
                    .placeholder("Search...")
                    .inline_list(true)
                    .open(true)
                    .value(picked)
                    .search(search)
                    .on_search_change(move |next| search.revise(|query| *query = next))
                    .options(choices)
                    .empty_message("Nothing found...")
                    .on_value_change(move |next| {
                        picked.revise(|current| {
                            *current = if allow_deselect && *current == next {
                                None
                            } else {
                                next
                            };
                        });
                        open.revise(|value| *value = false);
                    });
                if options == Options::Limited {
                    picker = picker.limit(5);
                } else {
                    picker = picker.filter(sapid::ComboboxFilter::None);
                }
                if let Some(name) = form_name {
                    picker = picker.form_name(name);
                }
                view()
                    .class(if options == Options::Limited {
                        "site-framework-search-list"
                    } else {
                        "site-framework-scroll-list"
                    })
                    .fill(picker)
            })
            .show_in(scope)
    })
}
