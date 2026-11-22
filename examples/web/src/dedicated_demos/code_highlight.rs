use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
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
        sapid::code_highlight()
            .theme(theme.clone())
            .name("Welcome function")
            .language(CodeLanguage::Rust)
            .source(
                "pub fn welcome(name: &str) -> String {\n    format!(\"Welcome, {name}!\")\n}\n",
            )
            .class("showcase-code-sample")
            .copy_button(copy)
            .show_in(scope)
    })
}
