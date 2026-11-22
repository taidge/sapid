use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example choice-demos-field")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(widget(move |scope| {
            let value = scope.cage(String::new());
            type StrengthRule = (&'static str, fn(&str) -> bool);
            let rules: [StrengthRule; 5] = [
                ("Includes at least 6 characters", |s| s.chars().count() >= 6),
                ("Includes number", |s| s.chars().any(|c| c.is_ascii_digit())),
                ("Includes lowercase letter", |s| {
                    s.chars().any(|c| c.is_ascii_lowercase())
                }),
                ("Includes uppercase letter", |s| {
                    s.chars().any(|c| c.is_ascii_uppercase())
                }),
                ("Includes special symbol", |s| {
                    s.chars().any(|c| "$&+,:;=?@#|'<>.^*()%!-".contains(c))
                }),
            ];
            let progress = sapid::progress(value.map(move |v| {
                (rules.iter().filter(|(_, test)| test(v)).count() as f64 * 20.).max(10.)
            }))
            .name("Password strength");
            let mut checks: Vec<Box<dyn Widget>> = vec![Box::new(progress)];
            for (label, test) in rules {
                checks.push(Box::new(
                    text()
                        .class(value.map(move |v| {
                            if test(v) {
                                "choice-rule-met"
                            } else {
                                "choice-rule-missing"
                            }
                            .to_owned()
                        }))
                        .content(label),
                ));
            }
            let panel = sapid::stack()
                .gap(5.)
                .class("choice-strength-panel")
                .fill(checks);
            view()
                .class("choice-strength-target")
                .fill((
                    sapid::field("strength-password")
                        .theme(theme.clone())
                        .label("Your password")
                        .required(true)
                        .control(move |f| {
                            sapid::password_input()
                                .field(f)
                                .placeholder("Your password")
                                .value(value)
                                .on_value_change(move |next| value.revise(|v| *v = next))
                        }),
                    panel,
                ))
                .show_in(scope)
        }))
}
