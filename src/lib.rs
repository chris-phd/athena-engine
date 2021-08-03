use wasm_bindgen::prelude::*;
use web_sys::console;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str("[lib::main_js]: Athena Engine"));

    Ok(())
}

//
// The functions that the JavaScript front end interfaces with
//

#[wasm_bindgen]
pub fn new_game(board_setup : usize, players : usize, is_white_turn : bool) {
    console::log_1(&JsValue::from_str("[lib::new_game]: "));


}

#[wasm_bindgen]
pub fn update_board() -> bool {
    // Returns is_update_sucessful. If update fails, the front end is notified
    // that the last move failed.

    console::log_1(&JsValue::from_str("[lib::update_board]: "));

    return true;
}

#[wasm_bindgen]
pub fn ai_player_move() {

    console::log_1(&JsValue::from_str("[lib::ai_player_move]: "));

}