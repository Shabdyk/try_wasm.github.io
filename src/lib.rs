mod tictactoe;

use std::cell::RefCell;
use tictactoe::*;
use wasm_bindgen::prelude::*;

thread_local! {
  static TICTACTOE: RefCell<Tictactoe>
    = RefCell::new(Tictactoe::new(10, 10));
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

//
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }