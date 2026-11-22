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
        .fill(
            view().class("site-dd-width-200").fill(
                sapid::field("select-width-field")
                    .theme(theme.clone())
                    .label("Your favorite library")
                    .control(move |field| {
                        sapid::select("select-width")
                            .placeholder("Pick value")
                            .options(
                                ["React", "Angular", "Vue", "Svelte"]
                                    .map(|label| sapid::select_option(label, label)),
                            )
                            .field(field)
                    }),
            ),
        )
}
