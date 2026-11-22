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
            sapid::field("select-prefix-filter-field")
                .theme(theme.clone())
                .label("Your favorite library")
                .control(move |field| {
                    sapid::combobox("select-prefix-filter")
                        .placeholder("Type the start of an option")
                        .filter(sapid::ComboboxFilter::Prefix)
                        .options(
                            ["React", "Angular", "Vue", "Svelte"]
                                .map(|label| sapid::select_option(label, label)),
                        )
                        .field(field)
                }),
        )
}
