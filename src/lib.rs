//https://dev.to/dandyvica/wasm-in-rust-without-nodejs-2e0c

mod password_builder;

use password_builder::generate_random_password;
use password_builder::PasswordOptions;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

#[wasm_bindgen]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn generate_password(options: u8, lenght: usize) -> String {
    let options = PasswordOptions::new(options, lenght);
    generate_random_password(&options)
}

#[wasm_bindgen]
pub fn show_new_password() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let lenght = document
        .get_element_by_id("nb_lenght")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<usize>()
        .unwrap();
    


    let options = get_options(&document);
    let password = generate_password(options, lenght);
    alert(&password);

    Ok(())
}

fn get_options(document: &web_sys::Document) -> u8 {
    let mut result = get_check_box_value(document, "cb_upper");
    result += get_check_box_value(document, "cb_lower");
    result += get_check_box_value(document, "cb_numeric");
    result += get_check_box_value(document, "cb_special");
    result
}

fn get_check_box_value(document: &web_sys::Document, id: &str) -> u8 {
    let check_box = document
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();
    if check_box.checked() {

        let value = check_box.get_attribute("data-value").and_then(|s| {
            match s.parse::<u8>(){
                Ok(val) => Some(val),
                Err(_) => None,
            }
        });
        value.unwrap_or_default()
    } else {
        0
    }
}
