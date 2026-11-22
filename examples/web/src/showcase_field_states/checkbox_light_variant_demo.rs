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
            sapid::stack().gap(7.).fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Light checkbox")
                    .class("site-choice-input choice-example choice-light")
                    .default_checked(true),
                sapid::mixed_checkbox()
                    .theme(theme.clone())
                    .label("Light indeterminate checkbox")
                    .class("site-choice-input choice-example choice-light")
                    .default_state(sapid::CheckState::Mixed),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Teal light checkbox")
                    .class("site-choice-input choice-example choice-light choice-teal")
                    .default_checked(true),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Grape light checkbox")
                    .class("site-choice-input choice-example choice-light choice-grape")
                    .default_checked(true),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Light checkbox with CSS color")
                    .class("site-choice-input choice-example choice-light choice-pink")
                    .default_checked(true),
            )),
        )
}
