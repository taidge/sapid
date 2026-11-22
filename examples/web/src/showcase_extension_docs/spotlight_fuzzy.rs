use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let query = scope.cage(String::new());
        let names = [
            "Open calendar",
            "Edit project",
            "Open settings",
            "Publish release notes",
            "Review tasks",
            "View file history",
        ];
        let results = scope.bond(move || {
            let search = query.get();
            let needle = search.to_lowercase();
            let matched = names
                .iter()
                .filter(|label| {
                    let text = label.to_lowercase();
                    let mut available = text.chars();
                    needle
                        .chars()
                        .all(|ch| available.by_ref().any(|next| next == ch))
                })
                .copied()
                .collect::<Vec<_>>();
            let actions = matched
                .into_iter()
                .map(|name| sapid::command_action(name.to_lowercase().replace(' ', "-"), name))
                .collect();
            sapid::CommandResponse {
                query: search,
                results: sapid::CommandResults::new(actions),
            }
        });
        sapid::spotlight("spotlight-fuzzy")
            .theme(theme.clone())
            .label("Open command search")
            .search(query)
            .on_search_change(move |value| query.revise(|s| *s = value))
            .results(results)
            .show_in(scope)
    })
}
