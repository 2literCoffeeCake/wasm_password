extern crate wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlInputElement};

pub fn get_lenght(document: &Document) -> usize {
    let lenght = document
        .get_element_by_id("nb_lenght")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<usize>()
        .unwrap();
    lenght
}

pub fn get_options(document: &Document) -> u8 {
    let mut result = get_check_box_value(document, "cb_upper");
    result += get_check_box_value(document, "cb_lower");
    result += get_check_box_value(document, "cb_numeric");
    result += get_check_box_value(document, "cb_special");
    result
}

fn get_check_box_value(document: &Document, id: &str) -> u8 {
    let check_box = document
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    if check_box.checked() {
        let value = check_box
            .get_attribute("data-value")
            .and_then(|s| match s.parse::<u8>() {
                Ok(val) => Some(val),
                Err(_) => None,
            });
        value.unwrap_or_default()
    } else {
        0
    }
}
