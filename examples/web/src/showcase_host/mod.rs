//! Browser history services for the Rust documentation application.
use crate::showcase_navigation::Page;
use glory::Cage;
#[cfg(any(target_arch = "wasm32", feature = "wry-showcase"))]
mod document;
#[cfg(all(not(target_arch = "wasm32"), feature = "wry-showcase"))]
pub mod native;
#[cfg(target_arch = "wasm32")]
use browser::report;
#[cfg(all(not(target_arch = "wasm32"), feature = "wry-showcase"))]
use native::report;
#[cfg(all(not(target_arch = "wasm32"), not(feature = "wry-showcase")))]
fn report(_: String) {}

pub fn request(request: sapid::AnchorRequest) {
    if let Err(error) = navigate(&request.href) {
        report(error);
    }
}
pub fn palette_request(request: sapid::AnchorRequest) {
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = browser::palette_navigate(&request.href) {
        report(error);
    }
    #[cfg(not(target_arch = "wasm32"))]
    self::request(request);
}
pub fn navigate(url: &str) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        browser::navigate(url)
    }
    #[cfg(all(not(target_arch = "wasm32"), feature = "wry-showcase"))]
    {
        native::navigate(url)
    }
    #[cfg(all(not(target_arch = "wasm32"), not(feature = "wry-showcase")))]
    {
        let _ = url;
        Ok(())
    }
}
pub fn present(page: &Page, source: Cage<Page>) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        browser::present(page, source)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (page, source);
        Ok(())
    }
}
#[cfg(target_arch = "wasm32")]
mod browser;
