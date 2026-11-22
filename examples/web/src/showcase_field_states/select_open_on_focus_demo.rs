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
        .fill(widget(move |scope| {
            let open = scope.cage(false);
            sapid::select("focus-open-select")
                .theme(theme.clone())
                .name("Your favorite library")
                .placeholder("Focus the field")
                .open(open)
                .on_open_change(move |next| open.revise(|v| *v = next))
                .on(glory::render::EventKind::FocusChanged, move |e| {
                    if let glory::render::Event::FocusChanged(true) = e {
                        open.revise(|v| *v = true)
                    }
                })
                .options(
                    ["React", "Angular", "Vue", "Svelte"]
                        .map(|label| sapid::select_option(label, label)),
                )
                .show_in(scope)
        }))
}
