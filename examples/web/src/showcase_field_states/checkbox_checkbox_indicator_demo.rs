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
            view()
                .class("choice-indicator-row")
                .semantics(Some(Box::new(glory::render::Semantics {
                    inert: true,
                    ..Default::default()
                })))
                .fill(
                    sapid::group().gap(16.).fill((
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("")
                            .class("site-choice-input choice-example "),
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("")
                            .class("site-choice-input choice-example ")
                            .default_checked(true),
                        sapid::mixed_checkbox()
                            .theme(theme.clone())
                            .label("")
                            .class("site-choice-input choice-example ")
                            .default_state(sapid::CheckState::Mixed),
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("")
                            .class("site-choice-input choice-example ")
                            .enabled(false),
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("")
                            .class("site-choice-input choice-example ")
                            .default_checked(true)
                            .enabled(false),
                        sapid::mixed_checkbox()
                            .theme(theme.clone())
                            .label("")
                            .class("site-choice-input choice-example ")
                            .default_state(sapid::CheckState::Mixed)
                            .enabled(false),
                    )),
                ),
        )
}
