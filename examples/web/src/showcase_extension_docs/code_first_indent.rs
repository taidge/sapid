use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let source = "    cargo run --example showcase \\\n      --features web";
    sapid::stack().theme(theme.clone()).gap(8.).fill((
        text().content("Dedent the first line"),
        sapid::code_highlight()
            .theme(theme.clone())
            .source(source)
            .dedent_first_line(true),
        text().content("Keep the first line indentation"),
        sapid::code_highlight().theme(theme).source(source),
    ))
}
