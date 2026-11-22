//! Launch the same Rust showcase in the native Glory Wry renderer.
use glory::{
    prelude::*,
    wry::shell::{Asset, Config, Shell, ShellError, WindowApp},
};
use sapid::Theme;
use sapid_web_example::{
    assets::media,
    showcase_catalog::{ENTRIES, Entry, find},
    showcase_host::native,
};
use std::{collections::BTreeMap, error::Error};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn mount(entry: &'static Entry, usage_only: bool) -> Result<WindowApp> {
    let window = WindowApp::new();
    window.renderer().enable_text_clipboard();
    window.renderer().enable_files()?;
    let theme = Theme::blue_light();
    if usage_only {
        window.mount(
            view()
                .class(format!("sapid-site site-light {}", theme.class()))
                .fill(
                    view()
                        .class("site-preview")
                        .fill(view().class("site-demo").fill((entry.build)(theme))),
                ),
        )?;
    } else {
        window.mount(native::site(theme, entry.slug.to_owned()))?;
    }
    window.app().settle(glory::ui::App::DEFAULT_SETTLE_PASSES)?;
    Ok(window)
}

fn assets() -> BTreeMap<String, Asset> {
    macro_rules! asset {
        ($name:literal, $mime:literal) => {
            (
                media($name),
                Asset {
                    mime: $mime.to_owned(),
                    bytes: include_bytes!(concat!("../../assets/", $name)).to_vec(),
                },
            )
        };
    }
    BTreeMap::from([
        asset!("alpine-lake-photo.png", "image/png"),
        asset!("team-member-photo.png", "image/png"),
        asset!("image-pattern.png", "image/png"),
        asset!("image-fallback.png", "image/png"),
        asset!("image-invalid.png", "image/png"),
        asset!("mountain-evening.svg", "image/svg+xml"),
        asset!("sapid-logo.svg", "image/svg+xml"),
    ])
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let first = args.next();
    let usage_only = first.as_deref() == Some("--usage");
    let command = if usage_only { args.next() } else { first };
    let help = "showcase_wry [--usage] <widget-slug | --audit | --list>";
    if args.next().is_some() {
        return Err(help.into());
    }
    match command.as_deref() {
        Some("--list") => {
            for entry in ENTRIES {
                println!("{}", entry.slug);
            }
        }
        Some("--audit") => {
            let mut failed = 0;
            for entry in ENTRIES {
                match mount(entry, usage_only) {
                    Ok(_) => println!("{}\tmounted", entry.slug),
                    Err(error) => {
                        println!("{}\t{error}", entry.slug);
                        failed += 1;
                    }
                }
            }
            if failed > 0 {
                return Err(format!("{failed} showcase entries failed to mount").into());
            }
        }
        Some(slug) => {
            let entry = find(slug).ok_or_else(|| format!("Unknown showcase widget: {slug}"))?;
            let mut css = String::new();
            for theme in [
                Theme::blue_light(),
                Theme::blue_dark(),
                Theme::green_light(),
                Theme::green_dark(),
            ] {
                css.push_str(&theme.stylesheet()?);
            }
            css.push_str(sapid::STRUCTURAL_CSS);
            css.push_str(include_str!("../../site.css"));
            Shell::new().run_until(
                mount(entry, usage_only)?,
                Config {
                    title: format!("Sapid · {}", entry.name),
                    width: 1280.,
                    height: 900.,
                    css,
                    assets: assets(),
                    ..Config::default()
                },
                |window| {
                    native::finish_navigation(window)
                        .map_err(|error| ShellError(error.to_string()))?;
                    Ok(false)
                },
            )?;
        }
        None => return Err(help.into()),
    }
    Ok(())
}
