use glory::prelude::*;
use sapid::TransitionEffect as Effect;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let effects = [
            Effect::Fade,
            Effect::FadeUp,
            Effect::FadeDown,
            Effect::FadeLeft,
            Effect::FadeRight,
            Effect::Scale,
            Effect::ScaleY,
            Effect::ScaleX,
            Effect::SkewUp,
            Effect::SkewDown,
            Effect::RotateLeft,
            Effect::RotateRight,
            Effect::SlideDown,
            Effect::SlideUp,
            Effect::SlideLeft,
            Effect::SlideRight,
            Effect::Pop,
            Effect::PopBottomLeft,
            Effect::PopBottomRight,
            Effect::PopTopLeft,
            Effect::PopTopRight,
        ];
        sapid::group()
            .theme(theme.clone())
            .justify(Justify::Center)
            .gap(8.)
            .fill(
                effects
                    .into_iter()
                    .map(|effect| {
                        let open = scope.cage(true);
                        let palette = theme.clone();
                        sapid::transition()
                            .theme(theme.clone())
                            .effect(effect)
                            .mounted(open)
                            .duration(280)
                            .exit_duration(260)
                            .on_exited(move || open.revise(|v| *v = true))
                            .content(move || {
                                sapid::button()
                                    .theme(palette.clone())
                                    .class("sapid-demo-effect-pill")
                                    .variant(sapid::ButtonVariant::Light)
                                    .compact(true)
                                    .name(effect.as_str())
                                    .label(effect.as_str().to_uppercase())
                                    .on_press(move || open.revise(|v| *v = false))
                            })
                    })
                    .collect::<Vec<_>>(),
            )
            .show_in(scope)
    })
}
