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
                    "tags-tags-menu-scroll-0",
                    "With scroll area (default)",
                    "Pick value or enter anything",
                    (1..=60)
                        .map(|i| Suggestion::new(format!("Option {i}")))
                        .collect(),
                    SuggestionMenu {
                        limit: 60,
                        class: "tags-menu-scroll",
                        externally_controlled: false,
                    },
                )
            },
            {
                use super::tag_suggestions::{Suggestion, SuggestionMenu, suggestions};
                suggestions(
                    theme.clone(),
                    "tags-tags-menu-scroll-1",
                    "With native scroll",
                    "Pick value or enter anything",
                    (1..=60)
                        .map(|i| Suggestion::new(format!("Option {i}")))
                        .collect(),
                    SuggestionMenu {
                        limit: 60,
                        class: "tags-menu-scroll tags-native-scroll",
                        externally_controlled: false,
                    },
                )
            },
        ))
}
