use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let handle = scope.cage(None::<sapid::ModalsHandle>);
        let weak = scope.downgrade();
        let palette = theme.clone();
        sapid::stack().theme(theme.clone()).fill((sapid::button().theme(theme.clone()).label("Delete project").on_press(move||{let(Some(h),Some(owner))=(handle.get(),weak.upgrade())else{return};let _=&palette;let result=(||->glory::ui::Result{h.open(sapid::confirmation_request("Delete project?","This example asks for confirmation before removing the project.").confirm_label("Delete project").close_label("Keep project"))?;Ok(())})();if let Err(e)=result{owner.report_error(e)}}),sapid::modals_manager("modal-confirm-style").theme(theme.clone()).stacked(false).on_ready(move|h|handle.revise(|v|*v=Some(h))))).show_in(scope)
    })
}
