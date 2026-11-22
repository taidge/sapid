use glory::prelude::*;
use sapid::{
    AffixHorizontal::{Left, Right},
    AffixOffset,
    AffixVertical::{Bottom, Top},
    NotificationsHandle,
};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let mut buttons = Vec::new();
        let mut outlets = Vec::new();
        for (position, horizontal, vertical) in [
            ("top-left", Left(16.), Top(16.)),
            ("top-right", Right(16.), Top(16.)),
            ("bottom-left", Left(16.), Bottom(16.)),
            ("bottom-right", Right(16.), Bottom(16.)),
            ("top-center", Left(0.), Top(16.)),
            ("bottom-center", Left(0.), Bottom(16.)),
        ] {
            let handle = scope.cage(None::<NotificationsHandle>);
            let owner = scope.downgrade();
            buttons.push(
                sapid::button()
                    .theme(theme.clone())
                    .label(position)
                    .on_press(move || {
                        let (Some(handle), Some(owner)) = (handle.get(), owner.upgrade()) else {
                            return;
                        };
                        if let Err(error) = handle.show(sapid::notification_request(
                            position,
                            "Notification",
                            format!("This notification is displayed at the {position}."),
                        )) {
                            owner.report_error(error);
                        }
                    }),
            );
            outlets.push(
                sapid::affix(format!("notification-{position}"))
                    .theme(theme.clone())
                    .class(if position.ends_with("center") {
                        "showcase-notice-outlet is-centered"
                    } else {
                        "showcase-notice-outlet"
                    })
                    .offset(AffixOffset {
                        horizontal,
                        vertical,
                    })
                    .style(
                        Style::new()
                            .width(Length::Px(320.))
                            .max_width(glory::style::Limit::Percent(90.)),
                    )
                    .fill(
                        sapid::toaster(format!("notify-{position}"))
                            .theme(theme.clone())
                            .visible_limit(1)
                            .on_ready(move |ready| handle.revise(|value| *value = Some(ready))),
                    ),
            );
        }
        glory::ui::view()
            .fill((
                sapid::group().theme(theme.clone()).fill(buttons),
                glory::ui::view().fill(outlets),
            ))
            .show_in(scope)
    })
}
