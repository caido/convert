//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use convert::encoding::url_encode::UrlEncode;
use convert::encoding::url_encode::WUrlEncode;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let x = WUrlEncode {
        url_encode: UrlEncode::new(false),
    };
    let value = "a@ test".as_bytes();
    let expected = "a%40%20test".as_bytes().to_vec();
    assert_eq!(x.apply(&value).unwrap(), expected);
}
