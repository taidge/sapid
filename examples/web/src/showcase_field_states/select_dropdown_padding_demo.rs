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
                sapid::field("select-padding-a-field")
                    .theme(theme.clone())
                    .label("No padding")
                    .control(move |field| {
                        sapid::select("select-padding-a")
                            .placeholder("Pick value")
                            .options(
                                ["React", "Angular", "Vue", "Svelte"]
                                    .map(|label| sapid::select_option(label, label)),
                            )
                            .field(field)
                    }),
            ),
            view().class("site-dd-padding-ten").fill(
                sapid::field("select-padding-b-field")
                    .theme(theme.clone())
                    .label("10px padding")
                    .control(move |field| {
                        sapid::select("select-padding-b")
                            .placeholder("Pick value")
                            .options(
                                ["React", "Angular", "Vue", "Svelte"]
                                    .map(|label| sapid::select_option(label, label)),
                            )
                            .field(field)
                    }),
            ),
        ))
}
