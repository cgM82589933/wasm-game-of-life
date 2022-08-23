//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}


#[wasm_bindgen_test]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
// #[[allow(dead_code)]
// fn bad_add(a: i32, b: i32) -> i32 {
//     a - b
// }

#[wasm_bindgen_test]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    // #[test]
    // fn test_bad_add() {
        // assert_eq!(bad_add(1, 2), -1);
    // }
}