const { greet } = wasm_bindgen;
async function run() {
    await wasm_bindgen('./pkg/without_a_bundler_no_modules_bg.wasm');
    greet()
}
run();