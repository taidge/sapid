mod carousel_cards;
mod carousel_container;
mod carousel_control_icons;
mod carousel_dots;
mod carousel_drag;
mod carousel_gap;
mod carousel_hover;
mod carousel_images;
mod carousel_inactive;
mod carousel_responsive;
mod carousel_vertical;
use super::showcase_examples::DetailSection;
mod autoplay;
mod controlled;
mod geometry;
mod retention;
pub fn navigation() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Responsive styles", "#carousel-responsive"),
        ("Container queries", "#carousel-container"),
        ("Controls icons", "#carousel-control-icons"),
        ("Drag free", "#carousel-drag"),
        ("Vertical orientation", "#carousel-vertical"),
        ("Size and gap", "#carousel-gap"),
        ("Show controls on hover", "#carousel-hover"),
        ("Hide inactive controls", "#carousel-inactive"),
        ("Indicator styles", "#carousel-dots"),
        ("Images carousel", "#carousel-images"),
        ("Cards carousel", "#carousel-cards"),
        ("Controlled pages", "#carousel-controlled"),
        ("Controls and direction", "#carousel-geometry"),
        ("Autoplay policy", "#carousel-autoplay"),
        ("Windowed content", "#carousel-retention"),
    ]
}
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    vec![
        DetailSection {
            id: "carousel-responsive",
            title: "Responsive styles",
            note: "Try the component in the preview.",
            example: Box::new(carousel_responsive::demo(theme.clone())),
            source: include_str!("carousel_responsive.rs"),
        },
        DetailSection {
            id: "carousel-container",
            title: "Container queries",
            note: "Try the component in the preview.",
            example: Box::new(carousel_container::demo(theme.clone())),
            source: include_str!("carousel_container.rs"),
        },
        DetailSection {
            id: "carousel-control-icons",
            title: "Controls icons",
            note: "Try the component in the preview.",
            example: Box::new(carousel_control_icons::demo(theme.clone())),
            source: include_str!("carousel_control_icons.rs"),
        },
        DetailSection {
            id: "carousel-drag",
            title: "Drag free",
            note: "Try the component in the preview.",
            example: Box::new(carousel_drag::demo(theme.clone())),
            source: include_str!("carousel_drag.rs"),
        },
        DetailSection {
            id: "carousel-vertical",
            title: "Vertical orientation",
            note: "Try the component in the preview.",
            example: Box::new(carousel_vertical::demo(theme.clone())),
            source: include_str!("carousel_vertical.rs"),
        },
        DetailSection {
            id: "carousel-gap",
            title: "Size and gap",
            note: "Try the component in the preview.",
            example: Box::new(carousel_gap::demo(theme.clone())),
            source: include_str!("carousel_gap.rs"),
        },
        DetailSection {
            id: "carousel-hover",
            title: "Show controls on hover",
            note: "Try the component in the preview.",
            example: Box::new(carousel_hover::demo(theme.clone())),
            source: include_str!("carousel_hover.rs"),
        },
        DetailSection {
            id: "carousel-inactive",
            title: "Hide inactive controls",
            note: "Try the component in the preview.",
            example: Box::new(carousel_inactive::demo(theme.clone())),
            source: include_str!("carousel_inactive.rs"),
        },
        DetailSection {
            id: "carousel-dots",
            title: "Indicator styles",
            note: "Try the component in the preview.",
            example: Box::new(carousel_dots::demo(theme.clone())),
            source: include_str!("carousel_dots.rs"),
        },
        DetailSection {
            id: "carousel-images",
            title: "Images carousel",
            note: "Try the component in the preview.",
            example: Box::new(carousel_images::demo(theme.clone())),
            source: include_str!("carousel_images.rs"),
        },
        DetailSection {
            id: "carousel-cards",
            title: "Cards carousel",
            note: "Try the component in the preview.",
            example: Box::new(carousel_cards::demo(theme.clone())),
            source: include_str!("carousel_cards.rs"),
        },
        DetailSection {
            id: "carousel-controlled",
            title: "Controlled pages and refusal",
            note: "current is a read-only input; on_current_change proposes a zero-based index. Disable acceptance to compare refused arrows, indicators and keyboard commands. Root arrows/Home/End and pointer or touch swipe share the same controller.",
            example: Box::new(controlled::demo(theme.clone())),
            source: include_str!("controlled.rs"),
        },
        DetailSection {
            id: "carousel-geometry",
            title: "Control size, direction and visibility",
            note: "with_controls(false) omits arrow buttons while keeping indicators, root keyboard navigation and swipe. control_size accepts finite 14–64px values. RTL reverses horizontal control placement; vertical changes the capture axis and uses up/down icons. The Options example shows neighboring slides with fractional sizing.",
            example: Box::new(geometry::demo(theme.clone())),
            source: include_str!("geometry.rs"),
        },
        DetailSection {
            id: "carousel-autoplay",
            title: "Explicit autoplay and pause policies",
            note: "Autoplay is opt-in. Enable this example to supply one cancellable two-second browser timer per tick. Focus, mouse/pen hover, the pause button and the external paused source stop the current wait. External pause disables local resume. SSR never constructs the timer; native applications supply their own local timer future.",
            example: Box::new(autoplay::demo(theme.clone())),
            source: include_str!("autoplay.rs"),
        },
        DetailSection {
            id: "carousel-retention",
            title: "Windowed slides and editor lifetime",
            note: "count supports up to 1,000,000 logical pages, overscan 0–3, with at most seven slide Owners. Indicators independently retain at most nine nearby positions. Enter a draft and move to a neighbor to inspect identity retention; jump out of the retained window to retire it. No visited-slide cache is kept.",
            example: Box::new(retention::demo(theme)),
            source: include_str!("retention.rs"),
        },
    ]
}
