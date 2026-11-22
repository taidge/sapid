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
        .fill({
            use super::tag_suggestions::{Suggestion, SuggestionMenu, suggestions};
            suggestions(
                theme.clone(),
                "tags-width",
                "Your favorite library",
                "Pick value or enter anything",
                ["React", "Angular", "Vue", "Svelte"]
                    .into_iter()
                    .map(Suggestion::new)
                    .collect(),
                SuggestionMenu {
                    limit: 99,
                    class: "tags-menu-width",
                    externally_controlled: false,
                },
            )
        })
}
