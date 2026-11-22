use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(4.).fill((
        text().content("Invoke"),
        sapid::code().theme(theme.clone()).content("main()"),
        text().content("to start the application."),
    ))
}
