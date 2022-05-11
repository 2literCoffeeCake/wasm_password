extern crate wasm_bindgen;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::{window, Document, HtmlInputElement};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = localStorage)]
    fn setItem(key: &str, value: &str);

    #[wasm_bindgen(js_namespace = localStorage)]
    fn getItem(key: &str) -> Option<String>;
}

static STORAGE_KEY: &str = "WASM_PASSWORD_";

pub fn set_item_to_client_storage(key: &str, value: &str) {
    let mut storage_key = String::from(STORAGE_KEY);
    storage_key.push_str(key);
    setItem(&storage_key, value);
}

pub fn get_item_from_client_storage(key: &str) -> Option<String> {
    let mut storage_key = String::from(STORAGE_KEY);
    storage_key.push_str(key);
    getItem(&storage_key)
}

pub fn get_document() -> Document {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    document
}

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
    let mut result = 0;
    for id in get_check_box_ids(){
        result += get_check_box_value(document, &id);
    }
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

pub fn get_check_box_ids() -> [String; 4] {
    [
        "cb_upper".to_owned(),
        "cb_lower".into(),
        "cb_numeric".into(),
        "cb_special".into(),
    ]
}
