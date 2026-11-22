use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let body = theme.clone();
    sapid::dialog("reference-modal-offset")
        .theme(theme)
        .title("Authentication")
        .class("site-reference-modal-offset")
        .label("Open modal")
        .content(move || controls::authentication(body.clone(), true))
}

#[allow(unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

    pub fn authentication(theme: sapid::Theme, interactive: bool) -> impl Widget {
        widget(move |scope| {
            let register = scope.cage(true);
            let accepted = scope.cage(true);
            let email = scope.cage(String::new());
            let password = scope.cage(String::new());
            let confirmation = scope.cage(String::new());
            let result = scope.cage(String::new());
            let first = theme.clone();
            let last = theme.clone();
            let mail_theme = theme.clone();
            let pass_theme = theme.clone();
            let extra_theme = theme.clone();
            let actions_theme = theme.clone();
            let prefix = if interactive {
                "modal-reference"
            } else {
                "loader-reference"
            };
            sapid::stack()
                .theme(theme.clone())
                .class("site-reference-auth")
                .gap(12.)
                .fill((
                    when(register, move || {
                        sapid::group()
                            .theme(first.clone())
                            .class("site-reference-auth-names")
                            .fill((
                                sapid::field(format!("{prefix}-first"))
                                    .theme(first.clone())
                                    .label("First name")
                                    .required(true)
                                    .control(|field| {
                                        sapid::text_field()
                                            .field(field)
                                            .placeholder("Your first name")
                                    }),
                                sapid::field(format!("{prefix}-last"))
                                    .theme(last.clone())
                                    .label("Last name")
                                    .required(true)
                                    .control(|field| {
                                        sapid::text_field()
                                            .field(field)
                                            .placeholder("Your last name")
                                    }),
                            ))
                    }),
                    sapid::field(format!("{prefix}-email"))
                        .theme(mail_theme.clone())
                        .label("Email")
                        .required(true)
                        .control(move |field| {
                            sapid::text_field()
                                .field(field)
                                .left_section(text().content("@"))
                                .placeholder("Your email")
                                .value(email)
                                .on_value_change(move |s| email.revise(|v| *v = s))
                        }),
                    sapid::field(format!("{prefix}-password"))
                        .theme(pass_theme.clone())
                        .label("Password")
                        .required(true)
                        .control(move |field| {
                            sapid::password_input()
                                .field(field)
                                .placeholder("Password")
                                .value(password)
                                .on_value_change(move |s| password.revise(|v| *v = s))
                        }),
                    when(register, move || {
                        sapid::stack().theme(extra_theme.clone()).gap(12.).fill((
                            sapid::field(format!("{prefix}-confirm"))
                                .theme(extra_theme.clone())
                                .label("Confirm Password")
                                .required(true)
                                .control(move |field| {
                                    sapid::password_input()
                                        .field(field)
                                        .placeholder("Confirm password")
                                        .value(confirmation)
                                        .on_value_change(move |s| confirmation.revise(|v| *v = s))
                                }),
                            sapid::checkbox()
                                .theme(extra_theme.clone())
                                .checked(accepted)
                                .on_checked_change(move |v| accepted.revise(|o| *o = v))
                                .label("I agree to sell my soul and privacy to this corporation"),
                        ))
                    }),
                    when(interactive, move || {
                        sapid::group()
                            .theme(actions_theme.clone())
                            .class("site-reference-auth-actions")
                            .fill((
                                sapid::button()
                                    .theme(actions_theme.clone())
                                    .variant(sapid::ButtonVariant::Subtle)
                                    .label(register.map(|r| {
                                        if *r {
                                            "Have an account? Login".to_owned()
                                        } else {
                                            "Don't have an account? Register".to_owned()
                                        }
                                    }))
                                    .on_press(move || {
                                        register.revise(|v| *v = !*v);
                                        result.revise(|v| v.clear())
                                    }),
                                sapid::button()
                                    .theme(actions_theme.clone())
                                    .label(register.map(|r| {
                                        if *r {
                                            "Register".to_owned()
                                        } else {
                                            "Login".to_owned()
                                        }
                                    }))
                                    .on_press(move || {
                                        result.revise(|v| {
                                            *v = if !email.get().contains('@') {
                                                "Enter a valid email address."
                                            } else if password.get().is_empty() {
                                                "Enter a password."
                                            } else if register.get()
                                                && (password.get() != confirmation.get()
                                                    || !accepted.get())
                                            {
                                                "Check your password confirmation and terms."
                                            } else {
                                                "Details accepted."
                                            }
                                            .to_owned()
                                        })
                                    }),
                            ))
                    }),
                    when(result.map(|v| !v.is_empty()), move || {
                        text().content(result)
                    }),
                ))
                .show_in(scope)
        })
    }
}
