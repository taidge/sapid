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
                "tags-viewport-float",
                "Fits viewport height",
                "Pick values",
                (1..=60)
                    .map(|n| Suggestion::new(format!("Option {n}")))
                    .collect(),
                SuggestionMenu {
                    limit: 99,
                    class: "tags-menu-viewport",
                    externally_controlled: false,
                },
            )
        })
}
