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
            view().class("site-dd-padding-zero").fill(
                sapid::field("autocomplete-padding-a-field")
                    .theme(theme.clone())
                    .label("No padding")
                    .control(move |field| {
                        sapid::autocomplete("autocomplete-padding-a")
                            .placeholder("Pick value or enter anything")
                            .options(
                                ["React", "Angular", "Vue", "Svelte"]
                                    .map(|label| sapid::select_option(label, label)),
                            )
                            .field(field)
                    }),
            ),
            view().class("site-dd-padding-ten").fill(
                sapid::field("autocomplete-padding-b-field")
                    .theme(theme.clone())
                    .label("10px padding")
                    .control(move |field| {
                        sapid::autocomplete("autocomplete-padding-b")
                            .placeholder("Pick value or enter anything")
                            .options(
                                ["React", "Angular", "Vue", "Svelte"]
                                    .map(|label| sapid::select_option(label, label)),
                            )
                            .field(field)
                    }),
            ),
        ))
}
