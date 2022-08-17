mod utils;

use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use utils::create_element;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use web_sys::HtmlButtonElement;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn use_counter() -> Result<HtmlButtonElement, JsValue> {
    let mut count = 0;

    // Create elements
    let button = create_element("button", HashMap::from([
        ("id","counter"),
        ("type","button"),
    ]));

    button.set_inner_html(format!("{}{}","Count is ",count).as_str());
        
    // Add the count on button click
    let click_event = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
        count += 1;
        let btn = event.current_target().unwrap().dyn_into::<web_sys::HtmlButtonElement>().unwrap();
        btn.set_inner_html(format!("{}{}","Count is ",count).as_str());
        unsafe {
            console::log_1(&"Clicked".into());
        }
    });

    button.add_event_listener_with_callback("click", click_event.as_ref().unchecked_ref()).ok();
    click_event.forget();

    unsafe {
        web_sys::console::log_1(&"Rendered".into());
    }
    
    Ok(button.dyn_into::<HtmlButtonElement>().ok().unwrap())
}
