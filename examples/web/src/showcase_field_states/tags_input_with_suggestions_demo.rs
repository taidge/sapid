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
                "tags-basic-suggestions",
                "Press Enter to submit a tag",
                "Pick tag from list",
                ["React", "Angular", "Svelte"]
                    .into_iter()
                    .map(Suggestion::new)
                    .collect(),
                SuggestionMenu {
                    limit: 99,
                    class: "",
                    externally_controlled: false,
                },
            )
        })
}
