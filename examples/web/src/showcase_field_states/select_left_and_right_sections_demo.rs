use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let ink = theme.muted;
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
            sapid::field("select-inset-start")
                .theme(theme.clone())
                .label("Your favorite library")
                .control(move |field| {
                    sapid::input_group()
                        .inset_start(
                            sapid::theme_icon(sapid::Icon::SquaresFour)
                                .icon_size(16.)
                                .color(ink)
                                .variant(sapid::ThemeIconVariant::Transparent),
                        )
                        .input(
                            sapid::select("select-inset-start-control")
                                .placeholder("Your favorite library")
                                .options(
                                    ["React", "Angular", "Vue"]
                                        .map(|label| sapid::select_option(label, label)),
                                )
                                .field(field),
                        )
                }),
            sapid::field("select-inset-end")
                .theme(theme.clone())
                .label("Your favorite library")
                .control(move |field| {
                    sapid::input_group()
                        .inset_end(
                            sapid::theme_icon(sapid::Icon::SquaresFour)
                                .icon_size(16.)
                                .color(ink)
                                .variant(sapid::ThemeIconVariant::Transparent),
                        )
                        .input(
                            sapid::select("select-inset-end-control")
                                .placeholder("Your favorite library")
                                .options(
                                    ["React", "Angular", "Vue"]
                                        .map(|label| sapid::select_option(label, label)),
                                )
                                .field(field),
                        )
                }),
        ))
}
