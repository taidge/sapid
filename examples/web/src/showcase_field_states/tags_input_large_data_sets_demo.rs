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
                "tags-large-data",
                "100 000 options tags input",
                "Use limit to optimize performance",
                (0..100_000)
                    .map(|n| Suggestion::new(format!("Option {n}")))
                    .collect(),
                SuggestionMenu {
                    limit: 5,
                    class: "",
                    externally_controlled: false,
                },
            )
        })
}
