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
                "tags-group-options",
                "Enter tags",
                "Enter tags",
                [
                    ("React", "Frontend"),
                    ("Angular", "Frontend"),
                    ("Express", "Backend"),
                    ("Django", "Backend"),
                ]
                .into_iter()
                .map(|(label, group)| Suggestion {
                    group,
                    ..Suggestion::new(label)
                })
                .collect(),
                SuggestionMenu {
                    limit: 99,
                    class: "",
                    externally_controlled: false,
                },
            )
        })
}
