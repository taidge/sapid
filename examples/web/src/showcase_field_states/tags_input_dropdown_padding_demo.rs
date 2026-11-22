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
        .fill((
            {
                use super::tag_suggestions::{Suggestion, SuggestionMenu, suggestions};
                suggestions(
                    theme.clone(),
                    "tags-tags-menu-zero-0",
                    "Zero padding",
                    "Pick value or enter anything",
                    ["React", "Angular", "Vue", "Svelte"]
                        .into_iter()
                        .map(Suggestion::new)
                        .collect(),
                    SuggestionMenu {
                        limit: 60,
                        class: "tags-menu-zero",
                        externally_controlled: false,
                    },
                )
            },
            {
                use super::tag_suggestions::{Suggestion, SuggestionMenu, suggestions};
                suggestions(
                    theme.clone(),
                    "tags-tags-menu-ten-1",
                    "10px padding",
                    "Pick value or enter anything",
                    ["React", "Angular", "Vue", "Svelte"]
                        .into_iter()
                        .map(Suggestion::new)
                        .collect(),
                    SuggestionMenu {
                        limit: 60,
                        class: "tags-menu-ten",
                        externally_controlled: false,
                    },
                )
            },
        ))
}
