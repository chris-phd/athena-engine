use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

/// A javascript console.log macro
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// Converts a chess coordinate (such as e4, f3) to a rank and file
/// Ranks and files start at 1 in the bottom left square (a1)
pub fn _coord_to_rank_file(coord : &str) -> [usize; 2] {
    assert!(coord.len() == 2);

    let mut chs = coord.bytes();
    let file_alpha = chs.next().unwrap();
    let rank_digit = chs.next().unwrap();
    let rank = (rank_digit - b'0') as usize;
    let file = (file_alpha - b'a') as usize;

    return [rank, file];
}