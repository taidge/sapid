use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let expanded = scope.cage(false);
        let source=expanded.map(|v| { if *v { "const PALETTE: [&str; 5] = [\n    \"blue\",\n    \"red\",\n    \"green\",\n    \"violet\",\n    \"orange\",\n];" } else { "const PALETTE: [&str; 5] = [\n    \"blue\",\n    \"red\"," }.to_owned() });
        sapid::stack()
            .theme(theme.clone())
            .gap(6.)
            .fill((
                sapid::code_highlight()
                    .theme(theme.clone())
                    .source(source)
                    .language(sapid::CodeLanguage::Rust),
                sapid::button()
                    .theme(theme.clone())
                    .label(expanded.map(|v| {
                        if *v {
                            "Show less".to_owned()
                        } else {
                            "Show full code".to_owned()
                        }
                    }))
                    .on_press(move || expanded.revise(|v| *v = !*v)),
            ))
            .show_in(scope)
    })
}
