use std::char;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn zen2han(str: &str) -> String {
    let src: Vec<char> = str.chars().collect();
    let mut dst = String::new();
    for c in src {
        let code = c as u32;
        if (0xFF10 <= code && code <= 0xFF19)
            || (0xFF21 <= code && code <= 0xFF3A)
            || (0xFF41 <= code && code <= 0xFF5A)
        {
            // ０-９: FF10-FF19
            // Ａ-Ｚ: FF21-FF3A
            // ａ-ｚ: FF41-FF5A
            dst.push(char::from_u32(code - 0xFEE0).unwrap());
        } else {
            dst.push(c);
        }
    }
    return dst;
}

#[test]
fn it_works() {
    assert_eq!(zen2han("１９６８年"), "1968年");
    assert_eq!(zen2han("ＺＥＮＫＡＫＵ/HANKAKU"), "ZENKAKU/HANKAKU");
    assert_eq!(zen2han("ｌｏｗｅｒｃａｓｅ"), "lowercase");
}
