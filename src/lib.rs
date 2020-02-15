use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen]
pub fn greet() {
    alert("Hello World From Rust!");
    log("LOGGING!")
}
