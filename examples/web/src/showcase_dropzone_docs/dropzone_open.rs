use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let control = scope.cage(None::<sapid::FileController>);
        let weak = scope.downgrade();
        sapid::stack()
            .theme(theme.clone())
            .gap(8.)
            .fill((
                super::image_dropzone::image_dropzone(theme.clone()).controller_ref(control),
                sapid::button()
                    .theme(theme.clone())
                    .label("Select files")
                    .on_press(move || {
                        if let Some(c) = control.get()
                            && let Err(error) = c.open_picker()
                            && let Some(scope) = weak.upgrade()
                        {
                            scope.report_error(error)
                        }
                    }),
            ))
            .show_in(scope)
    })
}
