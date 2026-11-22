use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let browsing = scope.cage(false);
        let action = theme.clone();
        sapid::group()
            .theme(theme.clone())
            .justify(Justify::Center)
            .fill(
                sapid::empty_state()
                    .theme(theme.clone())
                    .title("Your cart is empty")
                    .description("Add items to your cart to get started.")
                    .illustration(
                        sapid::center()
                            .theme(theme.clone())
                            .style(Style {
                                background: Some(Color(241, 243, 245, 255)),
                                ..Style::new()
                                    .width(Length::Px(96.))
                                    .height(Length::Px(96.))
                                    .radius(48.)
                            })
                            .fill(
                                glory::ui::graphic()
                                    .drawing(sapid::Icon::ShoppingCart.drawing(theme.muted))
                                    .style(
                                        Style::new().width(Length::Px(40.)).height(Length::Px(40.)),
                                    ),
                            ),
                    )
                    .actions(
                        sapid::button()
                            .theme(action)
                            .variant(sapid::ButtonVariant::Secondary)
                            .label(browsing.map(|v| {
                                if *v {
                                    "Products opened".to_owned()
                                } else {
                                    "Browse products".to_owned()
                                }
                            }))
                            .on_press(move || browsing.revise(|v| *v = true)),
                    ),
            )
            .show_in(scope)
    })
}
