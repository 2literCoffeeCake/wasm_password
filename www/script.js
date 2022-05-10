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

function on_value_change(el){
    let value = el.target.value;
    if(value.indexOf('.') > -1){
        el.target.value = parseInt(el.target.value)
    }

    value = parseInt(el.target.value);
    if(value < 3){
        el.target.value = "4";
    }
}

function init_components(){
    const inputs = document.getElementsByTagName('input');
    for(let i = 0; i < inputs.length; i++){
        inputs[i].disabled = false;
    }
    const button = document.getElementsByTagName('button')[0];
    button.disabled = false;
    //button.onclick = show_new_password;
    document.getElementById('nb_lenght').addEventListener('change', on_value_change);
}



document.addEventListener("DOMContentLoaded", function(){
    load_wasm().then(init_components);
});