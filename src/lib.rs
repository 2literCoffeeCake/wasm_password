mod password_builder;
use password_builder::{generate_random_password, PasswordOptions};

mod util;
use util::{get_check_box_ids, get_item_from_client_storage, set_item_to_client_storage};
use util::{get_document, get_lenght, get_options};

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{HtmlButtonElement, HtmlElement, HtmlInputElement, MouseEvent};

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

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    init_button()?;
    init_numeric_text_box()?;
    init_check_boxes()?;
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
    let id = "nb_lenght";
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
        set_item_to_client_storage(&format!("{}_value", id), &element.value());
    }) as Box<dyn FnMut(_)>);

    let input = get_document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
    closure.forget();
    let value = get_item_from_client_storage(&format!("{}_value", &id))
        .unwrap_or("8".to_owned())
        .parse::<f64>()
        .unwrap_or(8.0);
        input.set_value_as_number(value);
    Ok(())
}

fn init_check_boxes() -> Result<(), JsValue> {
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        let element = event
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        let mut value = "0";
        if element.checked() {
            value = "1";
        }
        set_item_to_client_storage(&format!("{}_value", element.id()), value);
    }) as Box<dyn FnMut(_)>);
    for id in get_check_box_ids() {
        let input = get_document()
            .get_element_by_id(&id)
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
        let value =
            get_item_from_client_storage(&format!("{}_value", &id)).unwrap_or("1".to_owned());
        let value = match value.as_str() {
            "1" => true,
            _ => false,
        };
        input.set_checked(value);
    }
    closure.forget();
    Ok(())
}

#[wasm_bindgen]
pub fn enable_components() -> Result<(), JsValue> {
    let nodes = get_document().query_selector_all("[data-ui-element=\"true\"]")?;
    let mut index = 0;
    loop {
        if index >= nodes.length() {
            break;
        }
        if let Some(node) = nodes.get(index) {
            let element = node.dyn_into::<HtmlElement>()?;
            let id = element.id();
            if &id == "button" {
                let element = element.dyn_into::<HtmlButtonElement>()?;
                element.set_disabled(false);
            } else {
                let element = element.dyn_into::<HtmlInputElement>()?;
                element.set_disabled(false);
            }
        };
        index += 1;
    }
    Ok(())
}