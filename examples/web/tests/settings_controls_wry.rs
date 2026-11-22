//! Exercises the actual system WebView. Set SAPID_WRY_WINDOW=1 to run it.
use glory::{
    prelude::{button, view},
    render::EventKind,
    style::{Length, Style},
    wry::shell::{Config, Shell, ShellError, WindowApp},
};
use std::{
    cell::Cell,
    rc::Rc,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("SAPID_WRY_WINDOW").as_deref() != Ok("1") {
        println!("Set SAPID_WRY_WINDOW=1 to test the system WebView.");
        return Ok(());
    }
    let result = Rc::new(Cell::new(None));
    let report = |class, value| {
        let result = result.clone();
        button()
            .class(class)
            .label(class)
            .on(EventKind::Activate, move |_| result.set(Some(value)))
    };
    let window = WindowApp::new();
    window.mount(
        view().fill((
            sapid::segmented_control("scheme")
                .label("Color scheme")
                .default_value("auto")
                .segment(sapid::segment("auto").label("自动"))
                .segment(sapid::segment("light").label("浅色"))
                .segment(sapid::segment("dark").label("深色模式")),
            sapid::native_select("interval")
                .class("interval")
                .name("Refresh interval")
                .style(Style::new().min_height(Length::Px(34.)))
                .default_value(Some("app".into()))
                .option(sapid::select_option("app", "跟随应用"))
                .option(sapid::select_option("fast", "每 5 秒")),
            report("test-passed", Ok(())),
            report("select-failed", Err("native select height")),
            report(
                "indicator-failed",
                Err("segmented indicator geometry or motion"),
            ),
        )),
    )?;
    let start = Instant::now();
    Shell::new().run_until(
        window,
        Config {
            title: "Sapid settings controls acceptance".into(),
            css: format!(
                "{}\n{}",
                sapid::STRUCTURAL_CSS,
                r#"
                .interval { width: 190px; appearance: auto; }
                .sapid-segmented-control { --sp-segment-duration: 400ms; }
            "#
            ),
            initialization_script: Some(include_str!("settings_controls_wry.js").into()),
            ..Config::default()
        },
        |_| match result.get() {
            Some(Ok(())) => Ok(true),
            Some(Err(reason)) => Err(ShellError(reason.into())),
            None if start.elapsed() > Duration::from_secs(10) => {
                Err(ShellError("settings controls acceptance timed out".into()))
            }
            _ => Ok(false),
        },
    )?;
    if result.get() != Some(Ok(())) {
        return Err("settings controls acceptance closed before completion".into());
    }
    println!("SAPID_WRY_SETTINGS_OK: native select height and segmented indicator frames");
    Ok(())
}
