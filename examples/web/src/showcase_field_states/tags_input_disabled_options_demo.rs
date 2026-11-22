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
                "tags-disabled-opts",
                "Enter tags",
                "Some tags are disabled",
                ["React", "Angular", "Vue", "Svelte"]
                    .into_iter()
                    .map(|label| Suggestion {
                        enabled: !matches!(label, "Vue" | "Svelte"),
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
