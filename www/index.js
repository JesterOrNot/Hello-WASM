const { greet } = wasm_bindgen;
async function run() {
    await wasm_bindgen('../pkg/hello_wasm_bg.wasm');
    greet()
}
run();