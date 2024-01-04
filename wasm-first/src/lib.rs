use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("result {}", name))
}

#[wasm_bindgen]
pub fn sum(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}