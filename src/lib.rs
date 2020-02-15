use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = document)]
    fn write(s: &str);
    #[wasm_bindgen(js_namespace = window)]
    fn prompt(s: &str) -> String;
}

// #[wasm_bindgen(start)]
// pub fn main() {
//     write("<h1>Hello World From Rust</h1>")
// }

#[wasm_bindgen]
pub fn greet() {
    log("LOGGING!");
    let s = prompt("Hello");
    let x = mossy::App::exec(String::from(&s));
    write(x.as_str());
}
