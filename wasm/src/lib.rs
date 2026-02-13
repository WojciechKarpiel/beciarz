use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn js_official_to_greek(input: &str) -> JsValue {
    JsValue::from_str(&beciarz_core::official_to_greek(input))
}

#[wasm_bindgen]
pub fn js_greek_to_official(input: &str) -> JsValue {
    JsValue::from_str(&beciarz_core::greek_to_official(input))
}
