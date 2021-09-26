extern crate wee_alloc;

use dominator::animation::timestamps;
use futures_signals::signal::{Mutable, SignalExt};
use signals_are_svelte_stores::*;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn raf() -> Derived {
    timestamps().map(|v| v.into()).into()
}

#[wasm_bindgen]
pub fn store() -> Writable {
    Mutable::new(JsValue::null()).into()
}
