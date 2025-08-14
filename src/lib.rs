use wasm_bindgen::prelude::*;

mod beciarz;

#[wasm_bindgen]
pub fn js_official_to_greek(input: &str) -> JsValue {
    JsValue::from_str(&beciarz::official_to_greek(input))
}
