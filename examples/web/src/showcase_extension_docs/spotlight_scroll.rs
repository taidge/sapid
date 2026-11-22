use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::spotlight("scroll-commands")
        .theme(theme)
        .label("Open action list")
        .actions(
            (1..=50)
                .map(|i| sapid::command_action(format!("file-{i}"), format!("Open document {i}"))),
        )
}
