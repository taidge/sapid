use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let chosen = scope.cage("Nothing selected".to_owned());
        let palette = theme.clone();
        sapid::stack()
            .gap(12.)
            .class("site-field-state-example site-combobox-button-target")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(340.)),
            )
            .fill((
                sapid::group().gap(4.).fill((
                    text()
                        .class("site-package-description")
                        .content("Selected item:"),
                    text().content(chosen),
                )),
                sapid::popover("button-choice-menu")
                    .theme(theme.clone())
                    .width(sapid::PopoverWidth::Px(200.))
                    .label("Pick item")
                    .open(open)
                    .show_close(false)
                    .on_open_change(move |next| open.revise(|v| *v = next))
                    .content(move || {
                        let options = [
                            ("apples", "🍎 Apples"),
                            ("bananas", "🍌 Bananas"),
                            ("broccoli", "🥦 Broccoli"),
                            ("carrots", "🥕 Carrots"),
                            ("chocolate", "🍫 Chocolate"),
                        ];
                        view().class("site-framework-scroll-list").fill(
                            sapid::combobox("button-choices")
                                .theme(palette.clone())
                                .name("Grocery options")
                                .open(true)
                                .inline_list(true)
                                .filter(sapid::ComboboxFilter::None)
                                .options(options.map(|(key, v)| sapid::select_option(key, v)))
                                .on_value_change(move |next| {
                                    if let Some((_, label)) =
                                        options.iter().find(|(k, _)| Some(*k) == next.as_deref())
                                    {
                                        chosen.revise(|v| *v = (*label).into());
                                        open.revise(|v| *v = false);
                                    }
                                }),
                        )
                    }),
            ))
            .show_in(scope)
    })
}
