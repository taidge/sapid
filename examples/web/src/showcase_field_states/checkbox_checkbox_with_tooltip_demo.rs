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
        .fill(sapid::stack().gap(7.).fill((
            {
                let theme = theme.clone();
                sapid::tooltip("checkbox-input-tip")
                    .theme(theme.clone())
                    .text("Checkbox with tooltip")
                    .target(move |target| {
                        target.view(
                            view()
                                .class("choice-tooltip-target choice-tooltip-input")
                                .fill(
                                    sapid::checkbox()
                                        .theme(theme.clone())
                                        .label("Tooltip on checkbox only")
                                        .class("site-choice-input choice-example "),
                                ),
                        )
                    })
            },
            {
                let theme = theme.clone();
                sapid::tooltip("checkbox-label-tip")
                    .theme(theme.clone())
                    .text("Checkbox with tooltip")
                    .target(move |target| {
                        target.view(
                            view().class("choice-tooltip-target ").fill(
                                sapid::checkbox()
                                    .theme(theme.clone())
                                    .label("Tooltip the entire element")
                                    .class("site-choice-input choice-example "),
                            ),
                        )
                    })
            },
        )))
}
