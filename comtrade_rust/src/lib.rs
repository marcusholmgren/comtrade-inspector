use comtrade::{ComtradeParserBuilder, DataFormat};
use serde::Serialize;
use std::{io::BufReader, panic};
use wasm_bindgen::prelude::*;

fn data_format_to_str(format: &DataFormat) -> &'static str {
    match format {
        DataFormat::Ascii => "ASCII",
        DataFormat::Binary16 => "BINARY",
        DataFormat::Binary32 => "BINARY32",
        DataFormat::Float32 => "FLOAT32",
    }
}

#[derive(Serialize)]
pub struct ComtradeInfo {
    pub station: String,
    pub recording_device_id: String,
    pub start_time: String,
    pub trigger_time: String,
    pub data_format: String,
    pub frequency: f64,
}

#[wasm_bindgen]
pub fn parse_comtrade(cfg_file: &[u8], dat_file: &[u8]) -> Result<JsValue, JsValue> {
    let cfg_reader = BufReader::new(cfg_file);
    let dat_reader = BufReader::new(dat_file);

    let result = panic::catch_unwind(move || {
        ComtradeParserBuilder::new()
            .cfg_file(cfg_reader)
            .dat_file(dat_reader)
            .build()
            .parse()
    });

    match result {
        Ok(Ok(comtrade)) => {
            let info = ComtradeInfo {
                station: comtrade.station_name.clone(),
                recording_device_id: comtrade.recording_device_id.clone(),
                start_time: comtrade.start_time.to_string(),
                trigger_time: comtrade.trigger_time.to_string(),
                data_format: data_format_to_str(&comtrade.data_format).to_string(),
                frequency: comtrade.line_frequency,
            };
            serde_wasm_bindgen::to_value(&info).map_err(|e| JsValue::from_str(&e.to_string()))
        }
        Ok(Err(e)) => Err(JsValue::from_str(&format!(
            "Error parsing COMTRADE file: {:?}",
            e
        ))),
        Err(_) => Err(JsValue::from_str(
            "A panic occurred while parsing the COMTRADE file. This may be due to a malformed file.",
        )),
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
