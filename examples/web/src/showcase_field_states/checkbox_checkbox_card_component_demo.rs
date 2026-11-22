use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    view()
        .class("site-field-state-example choice-demos-field choice-package-card")
        .fill((
            sapid::checkbox()
                .theme(theme.clone())
                .name("mantine/core")
                .class("site-choice-input choice-example choice-card"),
            sapid::stack().gap(4.).class("choice-card-copy").fill((
                text().class("choice-card-title").content("mantine/core"),
                text()
                    .class("choice-card-description")
                    .content("Core components library: inputs, buttons, overlays, etc."),
            )),
        ))
}
