use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let alternate = scope.cage(false);
        let source = scope.bond(move || {
            if alternate.get() {
                "fn main() { println!(\"Second version\"); }".to_owned()
            } else {
                "fn main() { println!(\"First version\"); }".to_owned()
            }
        });
        sapid::code_highlight()
            .theme(theme.clone())
            .source(source)
            .controls(
                sapid::group().theme(theme.clone()).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .compact(true)
                        .label("Switch version")
                        .on_press(move || alternate.revise(|v| *v = !*v)),
                    sapid::copy_button()
                        .theme(theme.clone())
                        .source(source)
                        .label("Copy current version"),
                )),
            )
            .show_in(scope)
    })
}
