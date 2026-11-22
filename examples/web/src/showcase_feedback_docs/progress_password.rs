use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(String::new());
        let strength = value.map(|password| {
            let len = password.chars().count();
            if len < 5 {
                return if len == 0 { 0_i32 } else { 10_i32 };
            }
            let criteria = [
                len > 5,
                password.chars().any(|c| c.is_ascii_digit()),
                password.chars().any(|c| c.is_ascii_lowercase()),
                password.chars().any(|c| c.is_ascii_uppercase()),
                password
                    .chars()
                    .any(|c| "$&+,:;=?@#|'<>.^*()%!-".contains(c)),
            ];
            (20 * criteria.into_iter().filter(|v| *v).count() as i32).max(10)
        });
        let palette = theme.clone();
        let segments = each(scope.bond(move || vec![strength.get()]))
            .key(|v| *v)
            .seed(move |score| {
                let color = if score < 30 {
                    Color(250, 82, 82, 255)
                } else if score < 50 {
                    Color(253, 126, 20, 255)
                } else if score < 70 {
                    Color(250, 176, 5, 255)
                } else {
                    Color(18, 184, 134, 255)
                };
                let mut tone = palette.clone();
                tone.accent = color;
                sapid::group()
                    .theme(palette.clone())
                    .gap(5.)
                    .wrap(glory::style::FlexWrap::NoWrap)
                    .fill(
                        [1, 30, 50, 70]
                            .into_iter()
                            .enumerate()
                            .map(|(i, threshold)| {
                                sapid::progress(if score >= threshold { 100. } else { 0. })
                                    .theme(tone.clone())
                                    .name(format!("Password strength segment {}", i + 1))
                                    .thickness(4.)
                                    .style(Style {
                                        grow: Some(1.),
                                        ..Style::new().width(Length::Px(0.))
                                    })
                            })
                            .collect::<Vec<_>>(),
                    )
            });
        let input_theme = theme.clone();
        let password = sapid::field("strength-password")
            .theme(theme.clone())
            .label("Enter password")
            .control(move |field| {
                sapid::password_input()
                    .theme(input_theme.clone())
                    .field(field)
                    .placeholder("Enter password")
                    .value(value)
                    .on_value_change(move |s| value.revise(|v| *v = s))
            });
        sapid::group()
            .theme(theme.clone())
            .justify(Justify::Center)
            .fill(
                sapid::stack()
                    .theme(theme.clone())
                    .gap(10.)
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(Limit::Px(340.)),
                    )
                    .fill((password, segments)),
            )
            .show_in(scope)
    })
}
