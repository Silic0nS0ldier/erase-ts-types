use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn erase(ts_source: String) -> String {
    return "Hello, World!".to_string();
}

#[cfg(test)]
mod tests {
    use crate::erase;

    #[test]
    fn it_works() {
        assert_eq!(erase("".to_string()), "Hello, World!");
    }
}
