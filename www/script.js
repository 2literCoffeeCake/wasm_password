import { generate_password, default as init } from './wasm_password.js';
  
async function load_wasm() {
    try{
        console.log("Loading wasm...");
        await init('./wasm_password_bg.wasm');
        console.log("Wasm was loaded successfully");
    }catch(e){
        console.error(e);
    }

}

function init_components(){
    const inputs = document.getElementsByTagName('input');
    for(let i = 0; i < inputs.length; i++){
        inputs[i].disabled = false;
    }
    const button = document.getElementsByTagName('button')[0];
    button.disabled = false;
    button.onclick = () => {
        let options = 0;
        options += document.getElementById("cb_upper").checked ? 8 : 0;
        options += document.getElementById("cb_lower").checked ? 4 : 0;
        options += document.getElementById("cb_numeric").checked ? 2 : 0;
        options += document.getElementById("cb_special").checked ? 1 : 0;
        console.log()
        let pw = generate_password(options, parseInt(document.getElementById('nb_lenght').value));
        window.alert(pw);
    }

    document.getElementById('nb_lenght').addEventListener('change', (el) => {
        let value = el.target.value;
        if(value.indexOf('.') > -1){
            el.target.value = parseInt(el.target.value)
        }

        value = parseInt(el.target.value);
        if(value < 3){
            el.target.value = "4";
        }
    });


}

document.addEventListener("DOMContentLoaded", function(){
    load_wasm().then(init_components);
    
});