use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(true);
        let sidebar = sapid::sidebar("shell-alternative-layout-nav")
            .width(180.)
            .collapsed(open.map(|v| !*v))
            .name("Navbar")
            .content(|ctx| sapid::text().theme(ctx.theme).content("Navbar"));
        sapid::app_shell("shell-alternative-layout")
            .theme(theme.clone())
            .class("site-ref-shell site-ref-alternate")
            .breakpoint(480.)
            .sidebar(sidebar)
            .header(move |ctx| {
                sapid::group().theme(ctx.theme.clone()).gap(12.).fill((
                    sapid::button()
                        .theme(ctx.theme.clone())
                        .name("Toggle navbar")
                        .label("☰")
                        .pressed(open)
                        .on_press(move || open.revise(|v| *v = !*v)),
                    text().content("Header"),
                ))
            })
            .aside_width(130.)
            .aside(|ctx| sapid::text().theme(ctx.theme).content("Aside"))
            .main(|ctx| sapid::text().theme(ctx.theme).content("Main"))
            .footer(|ctx| sapid::text().theme(ctx.theme).content("Footer"))
            .show_in(scope)
    })
}
