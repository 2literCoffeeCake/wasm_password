//https://dev.to/dandyvica/wasm-in-rust-without-nodejs-2e0c

mod password_builder;
mod util;

use password_builder::generate_random_password;
use password_builder::PasswordOptions;

extern crate wasm_bindgen;
use util::get_lenght;
use util::get_options;
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

fn get_document() -> web_sys::Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    document
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    init_button()?;
    init_numeric_text_box()?;
    Ok(())
}

fn init_button() -> Result<(), JsValue> {
    let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        let document = get_document();
        let lenght = get_lenght(&document);
        let options = get_options(&document);
        let password = generate_password(options, lenght.to_owned());
        alert(&password);
    }) as Box<dyn FnMut(_)>);

    let document = get_document();
    let button = document
        .get_element_by_id("button")
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap();

    button.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn init_numeric_text_box() -> Result<(), JsValue> {
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let element = event
            .target()
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap();

        let raw_value = element.value();
        let mut value = raw_value.parse::<u32>().unwrap_or_else(|_|{
            match raw_value.parse::<f64>(){
                Ok(val) => {
                    val.floor() as u32
                },
                Err(_) => 4,
            }
        });
        if value < 4{
            value = 4;
        }
        element.set_value_as_number(value as f64);
    }) as Box<dyn FnMut(_)>);

    let input = get_document()
        .get_element_by_id("nb_lenght")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();
    input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(())
}
