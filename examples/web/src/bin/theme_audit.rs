//! Check that a selected theme reaches every themed widget in each compiled demo.
use glory::ui::App;

fn main() -> std::process::ExitCode {
    // Some full showcase demos exceed the 1 MiB Windows main-thread stack in debug builds.
    // Keep the extra stack local to this tool and run both theme passes on the same thread.
    std::thread::Builder::new()
        .name("showcase-theme-audit".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(audit)
        .expect("start showcase theme audit")
        .join()
        .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
}

fn audit() -> std::process::ExitCode {
    let mut failures = Vec::new();
    for theme in [
        sapid::Theme::green_dark(),
        sapid::Theme::from_palette("audit-custom", sapid::ColorScheme::Light, sapid::BLUE.base)
            .unwrap(),
    ] {
        for entry in sapid_web_example::showcase_catalog::ENTRIES {
            if std::env::var_os("SAPID_THEME_AUDIT_TRACE").is_some() {
                eprintln!("{}: {}", theme.class(), entry.slug);
            }
            let app = App::memory();
            app.mount(sapid_web_example::showcase_examples::demo(
                entry.slug,
                theme.clone(),
            ))
            .unwrap();
            let mut pending = app.tree().roots().to_vec();
            let mut unexpected = Vec::new();
            while let Some(id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(id).unwrap();
                pending.extend(node.children.iter().copied());
                let descriptor = node.descriptor.view();
                if descriptor.classes.iter().any(|c| {
                    c.starts_with("sapid-theme-")
                        && (c.ends_with("-light") || c.ends_with("-dark"))
                        && c != &theme.class()
                }) && !descriptor.classes.iter().any(|c| c == "sapid-unstyled")
                {
                    unexpected.push((descriptor.classes.to_vec(), descriptor.name.clone()));
                }
            }
            if !unexpected.is_empty() {
                failures.push(serde_json::json!({"id":entry.id,"theme":theme.class(),"unexpected":unexpected}));
            }
        }
    }
    println!("{}", serde_json::to_string_pretty(&failures).unwrap());
    if failures.is_empty() {
        std::process::ExitCode::SUCCESS
    } else {
        std::process::ExitCode::FAILURE
    }
}
