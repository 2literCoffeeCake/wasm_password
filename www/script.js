import { enable_components, default as init } from './wasm_password.js';

async function load_wasm() {
    try {
        console.log("Loading wasm...");
        await init('./wasm_password_bg.wasm');
        console.log("Wasm was loaded successfully");
    } catch (e) {
        console.error(e);
    }
}

document.addEventListener("DOMContentLoaded", function () {
    load_wasm().then(enable_components);
});