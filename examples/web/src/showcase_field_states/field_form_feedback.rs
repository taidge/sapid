pub(super) fn submit(data: serde_json::Value) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace=console, js_name=log)]
            fn log(text: &str);
        }
        log(&data.to_string());
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = data;
}
