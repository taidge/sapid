use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let source = "fn greet(name: &str) {\n    println!(\"Hello, {name}!\");\n}";
    widget(move |scope| {
        let feedback = scope.cage(sapid::CopyState::Idle);
        let copy = sapid::copy_button()
            .theme(theme.clone())
            .label("Copy code")
            .state_ref(feedback)
            .class(feedback.map(|state| match state {
                sapid::CopyState::Written => "showcase-code-copy is-copied".to_owned(),
                sapid::CopyState::Failed
                | sapid::CopyState::Denied
                | sapid::CopyState::Unavailable => "showcase-code-copy copy-failed".to_owned(),
                _ => "showcase-code-copy".to_owned(),
            }));
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::code_highlight()
                    .theme(theme.clone())
                    .class("showcase-code-sample")
                    .language(sapid::CodeLanguage::Rust)
                    .source(source)
                    .copy_button(copy),
                sapid::code_highlight()
                    .theme(theme.clone())
                    .class("showcase-code-sample")
                    .language(sapid::CodeLanguage::Rust)
                    .source("// Without a copy button\nfn greet() {}"),
            ))
            .show_in(scope)
    })
}
