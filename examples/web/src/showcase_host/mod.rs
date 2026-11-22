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

/// Open the platform color sampler in response to the user's picker button.
pub fn eye_dropper() -> sapid::ColorEyeDropper {
    sapid::ColorEyeDropper::new(|| async {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::{JsCast, JsValue};
            use web_sys::js_sys::{Array, Function, Promise, Reflect, global};
            let constructor = Reflect::get(&global(), &"EyeDropper".into())
                .ok()
                .and_then(|value| value.dyn_into::<Function>().ok())
                .ok_or("Screen color sampling is unavailable in this browser")?;
            let picker = Reflect::construct(&constructor, &Array::new())
                .map_err(|_| "Could not create the screen color sampler")?;
            let open = Reflect::get(&picker, &"open".into())
                .ok()
                .and_then(|value| value.dyn_into::<Function>().ok())
                .ok_or("Could not open the screen color sampler")?;
            let pending = open
                .call0(&picker)
                .map_err(|_| "Could not start screen color sampling")?
                .dyn_into::<Promise>()
                .map_err(|_| "Invalid screen sampler result")?;
            let result = match pending.await {
                Ok(value) => value,
                Err(error)
                    if Reflect::get(&error, &"name".into())
                        .ok()
                        .and_then(|v| v.as_string())
                        .as_deref()
                        == Some("AbortError") =>
                {
                    return Ok(None);
                }
                Err(_) => return Err("Screen color sampling failed".to_string()),
            };
            let hex = Reflect::get(&result, &JsValue::from_str("sRGBHex"))
                .ok()
                .and_then(|v| v.as_string())
                .ok_or("Screen sampler did not return a color")?;
            sapid::parse_hex_color(&hex)
                .map(Some)
                .map_err(|_| "Screen sampler returned an invalid color".into())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Err("Screen color sampling requires a supported browser".to_string())
        }
    })
}
