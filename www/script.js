import { default as init } from './wasm_password.js';
  
async function load_wasm() {
    try{
        console.log("Loading wasm...");
        await init('./wasm_password_bg.wasm');
        console.log("Wasm was loaded successfully");
    }catch(e){
        console.error(e);
    }
}

function enable_components(){
    const inputs = document.getElementsByTagName('input');
    for(let i = 0; i < inputs.length; i++){
        inputs[i].disabled = false;
    }
    const button = document.getElementsByTagName('button')[0];
    button.disabled = false;
}

document.addEventListener("DOMContentLoaded", function(){
    load_wasm().then(enable_components);
});