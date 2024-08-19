use js_sys::BigInt;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn pass() {
    let formatted_negative_bigint = format!("{}", BigInt::from(-1));
    assert_eq!(formatted_negative_bigint, "-1");
    // error output:
    //     panicked at src/lib.rs:7:5:
    //     assertion `left == right` failed
    //       left: "--1"
    //      right: "-1"
}
