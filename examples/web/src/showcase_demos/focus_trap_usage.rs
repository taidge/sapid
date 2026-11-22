use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focus_form(theme, "focus-first", false)
}

fn focus_form(theme: sapid::Theme, key: &'static str, target_second: bool) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(false);
        let preferred = scope.cage(None::<glory::render::SemanticId>);
        let configuration = scope.bond(move || glory::render::FocusScopeConfig {
            initial_focus: preferred.get(),
            ..Default::default()
        });
        let toggle = sapid::button()
            .theme(theme.clone())
            .class("site-focus-form-toggle")
            .label(active.map(|on| {
                if *on {
                    String::from("Deactivate focus trap")
                } else {
                    String::from("Activate focus trap")
                }
            }))
            .on_press(move || active.revise(|v| *v = !*v));
        let fields = ["First input", "Second input", "Third input"]
            .into_iter()
            .enumerate()
            .map(|(index, name)| {
                sapid::field(format!("{key}-{index}"))
                    .theme(theme.clone())
                    .label(name)
                    .control(move |field| {
                        if target_second && index == 1 {
                            preferred.revise(|id| *id = Some(field.id.clone()));
                        }
                        sapid::text_field().field(field).placeholder(name)
                    })
            })
            .collect::<Vec<_>>();
        let form = sapid::focus_scope()
            .configuration(configuration)
            .active(active)
            .loop_tab(true)
            .contain_focus(true)
            .auto_focus(true)
            .restore_focus(true)
            .style(
                Style::new()
                    .display(Display::Flex)
                    .flex_direction(FlexDirection::Column)
                    .gap(12.),
            )
            .fill(fields);
        sapid::group()
            .theme(theme.clone())
            .justify(Justify::Center)
            .fill(
                sapid::stack()
                    .theme(theme.clone())
                    .gap(12.)
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(Limit::Px(400.)),
                    )
                    .fill((toggle, form)),
            )
            .show_in(scope)
    })
}
