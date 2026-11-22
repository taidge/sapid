use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example choice-demos-centered")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            sapid::group().gap(16.).fill((
                sapid::switch()
                    .theme(theme.clone())
                    .name("On or off")
                    .size(sapid::Size::ExtraSmall)
                    .label("")
                    .with_thumb_indicator(false)
                    .class("choice-switch-inner"),
                sapid::switch()
                    .theme(theme.clone())
                    .name("On or off")
                    .size(sapid::Size::Small)
                    .label("")
                    .with_thumb_indicator(false)
                    .class("choice-switch-inner"),
                sapid::switch()
                    .theme(theme.clone())
                    .name("On or off")
                    .size(sapid::Size::Medium)
                    .label("")
                    .with_thumb_indicator(false)
                    .class("choice-switch-inner"),
                sapid::switch()
                    .theme(theme.clone())
                    .name("On or off")
                    .size(sapid::Size::Large)
                    .label("")
                    .with_thumb_indicator(false)
                    .class("choice-switch-inner"),
                sapid::switch()
                    .theme(theme.clone())
                    .name("On or off")
                    .size(sapid::Size::ExtraLarge)
                    .label("")
                    .with_thumb_indicator(false)
                    .class("choice-switch-inner"),
            )),
        )
}
