use wasm_bindgen::prelude::*;
use web_sys::console;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//
// The functions that the JavaScript front end interfaces with
//
#[wasm_bindgen]
pub struct GameState {

}

#[wasm_bindgen]
impl GameState {
    pub fn new() -> GameState {
        console::log_1(&JsValue::from_str("[rust::GameState::new]: "));
        return GameState {};
    }

    pub fn hello(&self) {
        console::log_1(&JsValue::from_str("[rust::GameState::hello]: Hello from a method in rust!"));
    }
}

#[wasm_bindgen]
pub fn hello() {
    console::log_1(&JsValue::from_str("[rust::hello]: Hello from a function in rust!"));
}