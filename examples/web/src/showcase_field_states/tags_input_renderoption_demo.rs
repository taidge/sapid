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
                "tags-grocery-detail",
                "Groceries",
                "Pick tag from list or type to add new",
                [
                    ("Apples", "🍎 Crisp and juicy snacking delight"),
                    ("Bread", "🍞 Freshly baked daily essential"),
                    ("Bananas", "🍌 Perfect for a healthy breakfast"),
                    ("Eggs", "🥚 Versatile protein source for cooking"),
                    ("Broccoli", "🥦 Nutrient-rich green vegetable"),
                ]
                .into_iter()
                .map(|(label, description)| Suggestion {
                    description,
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
