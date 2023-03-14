mod utils;

use js_sys::{BigInt, Boolean, JsString, Number, Object, Symbol};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn what(arg: JsValue) -> JsValue {
    get_type(&arg)
}

#[wasm_bindgen]
pub fn wot(arg: JsValue) -> JsValue {
    arg.js_typeof()
}

#[wasm_bindgen]
pub fn v2(arr: Vec<JsValue>) -> Vec<JsValue> {
    let mut types: Vec<JsValue> = Vec::new();
    for i in 0..arr.len() {
        types.push(get_type(&arr[i]));
    }

    types
}

fn get_type(arg: &JsValue) -> JsValue {
    if arg.is_array() {
        JsValue::from_str("array")
    } else if arg.has_type::<JsString>() {
        JsValue::from_str("string")
    } else if arg.has_type::<BigInt>() {
        JsValue::from_str("bigint")
    } else if arg.has_type::<Symbol>() {
        JsValue::from_str("symbol")
    } else if arg.has_type::<Number>() {
        JsValue::from_str("number")
    } else if arg.has_type::<Object>() {
        JsValue::from_str("object")
    } else if arg.has_type::<Boolean>() {
        JsValue::from_str("boolean")
    } else if arg.is_undefined() {
        JsValue::from_str("undefined")
    } else if arg.is_null() {
        JsValue::from_str("null")
    } else {
        JsValue::from_str("not sure yet")
    }
}
