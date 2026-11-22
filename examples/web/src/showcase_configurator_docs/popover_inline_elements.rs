use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    view().class("site-reference-inline-text site-reference-inline-popover").fill((
text().content("Stantler’s magnificent antlers were traded at high prices as works of art. As a result, this Pokémon was hunted close to extinction by those who were after the priceless antlers. "),
sapid::popover("reference-inline-popover").theme(theme).label("When visiting a junkyard").title("Inline dropdown").placement(glory::render::AnchorPlacement::new(glory::render::AnchorSide::Top,glory::render::AnchorAlign::Center)).content(||text().content("Inline dropdown")),
text().content(", you may catch sight of it having an intense fight with Murkrow over shiny objects.Ho-Oh’s feathers glow in seven colors depending on the angle at which they are struck by light. These feathers are said to bring happiness to the bearers. This Pokémon is said to live at the foot of a rainbow.")
))
}
