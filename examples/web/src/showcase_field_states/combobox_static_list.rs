use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let search = scope.cage(String::new());
        sapid::stack()
            .class("site-field-state-example site-inline-combobox-options")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(340.)),
            )
            .fill(
                sapid::combobox("inline-choices")
                    .theme(theme.clone())
                    .name("Pick value")
                    .placeholder("Pick value")
                    .search(search)
                    .on_search_change(move |next| search.revise(|v| *v = next))
                    .open(true)
                    .inline_list(true)
                    .filter(sapid::ComboboxFilter::None)
                    .options(["First", "Second", "Third"].map(|v| sapid::select_option(v, v))),
            )
            .show_in(scope)
    })
}
