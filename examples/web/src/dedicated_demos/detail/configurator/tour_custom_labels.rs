use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::TourContext::with_spotlight(scope, "custom-labels")?;
        let open = scope.cage(false);
        let current = scope.cage(0usize);
        let preview = sapid::stack()
            .theme(theme.clone())
            .class("site-ref-tour-form")
            .gap(16.)
            .fill((
                sapid::group()
                    .theme(theme.clone())
                    .class("site-reference-actions")
                    .fill((
                        sapid::tour_target(&context, "first").fill(
                            sapid::button()
                                .theme(theme.clone())
                                .label("Primer objetivo"),
                        ),
                        sapid::tour_target(&context, "second").fill(
                            sapid::text_field()
                                .theme(theme.clone())
                                .name("Segundo objetivo")
                                .placeholder("Segundo objetivo"),
                        ),
                    )),
                sapid::button()
                    .theme(theme.clone())
                    .label("Iniciar recorrido")
                    .on_press(move || {
                        current.revise(|s| *s = 0);
                        open.revise(|s| *s = true)
                    }),
            ));
        let panel = sapid::tour(&context, "custom-labels")
            .theme(theme.clone())
            .compact_navigation(true)
            .open(open)
            .current(current)
            .steps([
                sapid::tour_step("first", "Paso 1", "Este es el primer paso del recorrido."),
                sapid::tour_step("second", "Paso 2", "Este es el segundo paso del recorrido."),
            ])
            .navigation_labels("Anterior", "Siguiente", "Cerrar", "Omitir")
            .step_counter(|i, total| format!("{i} de {total}"))
            .on_current_change(move |v| current.revise(|old| *old = v))
            .on_open_change(move |v| open.revise(|old| *old = v));
        group().fill((preview, panel)).show_in(scope)
    })
}
