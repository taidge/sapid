use glory::prelude::*;
use glory::render::{FileConfig, FileFilter};
fn image_dropzone(theme: sapid::Theme) -> sapid::Dropzone {
    sapid::dropzone()
        .theme(theme)
        .class("showcase-image-picker")
        .name("Drop files here or choose files")
        .label("Drag images here or click to select files")
        .configuration(FileConfig {
            multiple: true,
            max_files: 32,
            max_file_bytes: 5 * 1024 * 1024,
            max_total_bytes: 160 * 1024 * 1024,
            filters: vec![FileFilter::Mime("image/*".into())],
            ..Default::default()
        })
        .description(|| {
            view().class("showcase-dropzone-message").fill((
                view().class("showcase-dropzone-icon"),
                view().class("showcase-dropzone-copy").fill((
                    text().content("Drag images here or click to select files"),
                    text().content("Attach up to 32 images, each no larger than 5 MiB."),
                )),
            ))
        })
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    image_dropzone(theme)
}
