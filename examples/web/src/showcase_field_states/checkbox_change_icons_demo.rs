use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example choice-demos-left")
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
                    .label("Custom icon")
                    .class("site-choice-input choice-example choice-hazard")
                    .default_checked(true),
                sapid::mixed_checkbox()
                    .theme(theme.clone())
                    .label("Custom icon: indeterminate")
                    .class("site-choice-input choice-example choice-hazard")
                    .default_state(sapid::CheckState::Mixed),
            )),
        )
}
