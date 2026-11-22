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
            view().class("site-dd-position").fill(
                sapid::field("combobox-popover-position-field")
                    .theme(theme.clone())
                    .label("Your favorite library")
                    .control(move |field| {
                        sapid::select("combobox-popover-position")
                            .placeholder("Pick value")
                            .options(
                                ["React", "Angular", "Vue", "Svelte"]
                                    .map(|label| sapid::select_option(label, label)),
                            )
                            .popup_placement(glory::render::AnchorPlacement::new(
                                glory::render::AnchorSide::Top,
                                glory::render::AnchorAlign::Start,
                            ))
                            .field(field)
                    }),
            ),
        )
}
