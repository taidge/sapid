use glory::prelude::*;

pub(super) fn image_dropzone(theme: sapid::Theme) -> sapid::Dropzone {
    sapid::dropzone()
        .theme(theme)
        .class("showcase-image-picker")
        .name("Drop files here or choose files")
        .label("Drag images here or click to select files")
        .configuration(glory::render::FileConfig {
            multiple: true,
            max_files: 32,
            max_file_bytes: 5 * 1024 * 1024,
            max_total_bytes: 160 * 1024 * 1024,
            filters: vec![glory::render::FileFilter::Mime("image/*".into())],
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
