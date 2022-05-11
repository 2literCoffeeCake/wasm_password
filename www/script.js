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
    const elements = document.querySelectorAll('[data-ui-element="true"]');
    for(let i = 0; i < elements.length; i++){
        elements[i].disabled = false;
    }
}

document.addEventListener("DOMContentLoaded", function(){
    load_wasm().then(enable_components);
});