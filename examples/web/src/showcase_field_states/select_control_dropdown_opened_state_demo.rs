use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        sapid::stack()
            .gap(16.)
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(340.)),
            )
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Toggle dropdown")
                    .on_press(move || open.revise(|current| *current = !*current)),
                sapid::field("select-external-open-field")
                    .theme(theme.clone())
                    .label("Your favorite library")
                    .control(move |field| {
                        sapid::select("select-external-open")
                            .placeholder("Pick value")
                            .options(
                                ["React", "Angular", "Vue", "Svelte"]
                                    .map(|label| sapid::select_option(label, label)),
                            )
                            .open(open)
                            .field(field)
                    }),
            ))
            .show_in(scope)
    })
}
