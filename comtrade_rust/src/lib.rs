// comtrade_rust/src/lib.rs
// This file is the main entry point for the Rust code that is compiled to WebAssembly.
// This file exists to parse COMTRADE files and return the information to the Svelte frontend.
// RELEVANT FILES: app/src/routes/info/+page.svelte

use comtrade::{AnalogChannel, ComtradeParserBuilder, DataFormat};
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

#[derive(Serialize, Clone)]
pub struct SerializableAnalogChannel {
    pub index: u32,
    pub name: String,
    pub units: String,
    pub min_value: f64,
    pub max_value: f64,
    pub multiplier: f64,
    pub offset_adder: f64,
    pub phase: String,
    pub circuit_component_being_monitored: String,
}

impl From<&AnalogChannel> for SerializableAnalogChannel {
    fn from(channel: &AnalogChannel) -> Self {
        Self {
            index: channel.index,
            name: channel.name.clone(),
            units: channel.units.clone(),
            min_value: channel.min_value,
            max_value: channel.max_value,
            multiplier: channel.multiplier,
            offset_adder: channel.offset_adder,
            phase: channel.phase.clone(),
            circuit_component_being_monitored: channel.circuit_component_being_monitored.clone(),
        }
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
    pub analog_channels: Vec<SerializableAnalogChannel>,
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
            let analog_channels: Vec<SerializableAnalogChannel> = comtrade
                .analog_channels
                .iter()
                .map(SerializableAnalogChannel::from)
                .collect();
            let info = ComtradeInfo {
                station: comtrade.station_name.clone(),
                recording_device_id: comtrade.recording_device_id.clone(),
                start_time: comtrade.start_time.to_string(),
                trigger_time: comtrade.trigger_time.to_string(),
                data_format: data_format_to_str(&comtrade.data_format).to_string(),
                frequency: comtrade.line_frequency,
                analog_channels,
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
