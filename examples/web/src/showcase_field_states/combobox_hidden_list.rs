use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let values = [
            "🍎 Apples",
            "🍌 Bananas",
            "🥦 Broccoli",
            "🥕 Carrots",
            "🍫 Chocolate",
        ];
        let search = scope.cage(String::new());
        let expanded = scope.cage(false);
        let has_matches = search.map(move |v| {
            values
                .iter()
                .any(|s| s.to_lowercase().contains(&v.trim().to_lowercase()))
        });
        let visible = scope.bond(move || expanded.get() && has_matches.get());
        sapid::field("hide-empty-choices")
            .theme(theme.clone())
            .class("site-field-state-example")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(340.)),
            )
            .label("Pick value or type anything")
            .control(move |field| {
                sapid::combobox("skip-empty")
                    .field(field)
                    .open(visible)
                    .search(search)
                    .placeholder("Pick value or type anything")
                    .options(
                        values
                            .into_iter()
                            .enumerate()
                            .map(|(i, s)| sapid::select_option(format!("grocery-{i}"), s)),
                    )
                    .on_open_change(move |v| expanded.revise(|open| *open = v))
                    .on_search_change(move |v| {
                        search.revise(|old| *old = v);
                        expanded.revise(|old| *old = true);
                    })
            })
            .show_in(scope)
    })
}
