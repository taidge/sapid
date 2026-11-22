use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut lime = theme.clone();
    lime.accent = Color(169, 227, 75, 255);
    let mut contrast = lime.clone();
    contrast.on_accent = lime.contrasting_foreground(lime.accent, lime.on_accent);
    sapid::stack().theme(theme).gap(4.).fill((
        sapid::nav_link("default-lime")
            .theme(lime)
            .variant(sapid::NavLinkVariant::Filled)
            .current(true)
            .label("Default")
            .href("#usage"),
        sapid::nav_link("contrast-lime")
            .theme(contrast)
            .variant(sapid::NavLinkVariant::Filled)
            .current(true)
            .label("Auto contrast")
            .href("#usage"),
    ))
}
