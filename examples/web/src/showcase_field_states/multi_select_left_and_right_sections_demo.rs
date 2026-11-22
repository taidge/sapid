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
            sapid::field("multi-select-inset-start")
                .theme(theme.clone())
                .label("Your favorite libraries")
                .control(move |field| {
                    sapid::input_group()
                        .inset_start(
                            sapid::theme_icon(sapid::Icon::Search)
                                .icon_size(16.)
                                .variant(sapid::ThemeIconVariant::Transparent),
                        )
                        .input(
                            sapid::multi_select("multi-select-inset-start-control")
                                .placeholder("Pick value")
                                .options(
                                    ["React", "Angular", "Vue", "Svelte"]
                                        .map(|label| sapid::select_option(label, label)),
                                )
                                .field(field),
                        )
                }),
            sapid::field("multi-select-inset-end")
                .theme(theme.clone())
                .label("Your favorite libraries")
                .control(move |field| {
                    sapid::input_group()
                        .inset_end(
                            sapid::theme_icon(sapid::Icon::Search)
                                .icon_size(16.)
                                .variant(sapid::ThemeIconVariant::Transparent),
                        )
                        .input(
                            sapid::multi_select("multi-select-inset-end-control")
                                .placeholder("Pick value")
                                .options(
                                    ["React", "Angular", "Vue", "Svelte"]
                                        .map(|label| sapid::select_option(label, label)),
                                )
                                .field(field),
                        )
                }),
        ))
}
