use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::anchor()
        .theme(theme.clone())
        .name("Open project details")
        .href("#card-sections")
        .fill(
            sapid::card()
                .theme(theme.clone())
                .bordered(true)
                .title("Project overview")
                .description("Open the project details")
                .fill(
                    sapid::text()
                        .theme(theme)
                        .content("Open the workspace summary"),
                ),
        )
}
