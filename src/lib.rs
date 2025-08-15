use wasm_bindgen::prelude::*;

mod beciarz;

#[wasm_bindgen]
pub fn js_official_to_greek(input: &str) -> JsValue {
    JsValue::from_str(&beciarz::official_to_greek(input))
}

#[wasm_bindgen]
pub fn js_greek_to_official(input: &str) -> JsValue {
    JsValue::from_str(&beciarz::greek_to_official(input))
}
