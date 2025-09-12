use comtrade::{Comtrade, ComtradeParserBuilder};
use std::io::BufReader;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[wasm_bindgen]
pub fn parse_comtrade(cfg_file: &[u8], dat_file: &[u8]) -> Result<JsValue, JsValue> {
    let cfg_reader = BufReader::new(cfg_file);
    let dat_reader = BufReader::new(dat_file);

    let result = ComtradeParserBuilder::new()
        .cfg_file(cfg_reader)
        .dat_file(dat_reader)
        .build()
        .parse();

    match result {
        Ok(comtrade) => {
            // You'll need to add serde and serde-wasm-bindgen to your Cargo.toml
            // and derive Serialize on the Comtrade struct to use this.
            // For now, we'll just return a simple success message.
            Ok(JsValue::from_str("Successfully parsed COMTRADE file!"))
        }
        Err(e) => Err(JsValue::from_str(&format!(
            "Error parsing COMTRADE file: {:?}",
            e
        ))),
    }
}
