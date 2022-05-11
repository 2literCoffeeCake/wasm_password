mod password_builder;
mod util;

use password_builder::generate_random_password;
use password_builder::PasswordOptions;

extern crate wasm_bindgen;
use util::get_lenght;
use util::get_options;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{window, Document, HtmlButtonElement, HtmlInputElement, MouseEvent};

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

fn get_document() -> Document {
    let window = window().expect("no global `window` exists");
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
    let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
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
        .dyn_into::<HtmlButtonElement>()
        .unwrap();

    button.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn init_numeric_text_box() -> Result<(), JsValue> {
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        let element = event
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();

        let mut value = element.value_as_number();
        value = value.floor();
        if value < 4.0 {
            value = 4.0;
        }
        element.set_value_as_number(value);
    }) as Box<dyn FnMut(_)>);

    let input = get_document()
        .get_element_by_id("nb_lenght")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(())
}
