cargo build --target wasm32-unknown-unknown
cd target\wasm32-unknown-unknown\debug
wasm-bindgen --target web --no-typescript --out-dir . wasm_password.wasm
wasm-gc wasm_password.wasm
cd ..\..\..
xcopy .\target\wasm32-unknown-unknown\debug\wasm_password_bg.wasm .\www /K /D /H /Y
xcopy .\target\wasm32-unknown-unknown\debug\wasm_password.js .\www /K /D /H /Y