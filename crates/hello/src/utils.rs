use std::collections::HashMap;

use js_sys::Array;
use wasm_bindgen::JsValue;
use web_sys::Element;

pub fn to_array(strings: &[&str] ) -> Array {
    let arr = Array::new_with_length(strings.len() as u32);
    for (i, s) in strings.iter().enumerate() {
        arr.set(i as u32, JsValue::from_str(s));
    }
    arr
}

pub fn create_element(name: &str, attributes: HashMap<&str, &str>) -> Element {
    let window = web_sys::window().expect("no global `window` exists");
    let document  = window.document().expect("should have `document` on a window");
    let el = document.create_element(name).expect("element not created");

    // Add the attributes
    for (key, value) in attributes.into_iter() {
        el.set_attribute(key.clone(), value.clone()).ok();
    }

    el
}