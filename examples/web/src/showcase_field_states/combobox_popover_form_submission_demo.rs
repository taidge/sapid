use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let picked = scope.cage(None);
        let submitted = scope.cage(String::new());
        sapid::form("framework-submission")
            .theme(theme.clone())
            .on_submit(move |_| {
                submitted.revise(|result| {
                    *result = picked
                        .get()
                        .map_or_else(String::new, |value| format!("Submitted value: {value}"));
                })
            })
            .fill(sapid::stack().gap(16.).fill((
                super::framework_popover::picker(
                    theme.clone(),
                    "framework-form",
                    picked,
                    "Select framework",
                    false,
                    super::framework_popover::Options::Frameworks,
                    Some("framework"),
                ),
                sapid::submit_button().theme(theme.clone()).label("Submit"),
                text().content(submitted),
            )))
            .show_in(scope)
    })
}
