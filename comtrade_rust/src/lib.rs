// comtrade_rust/src/lib.rs
// This file is the main entry point for the Rust code that is compiled to WebAssembly.
// This file exists to parse COMTRADE files and return the information to the Svelte frontend.
// RELEVANT FILES: app/src/routes/info/+page.svelte

use comtrade::{AnalogChannel, ComtradeParserBuilder, DataFormat};
use encoding_rs;
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
pub fn parse_comtrade(
    cfg_file: Option<Box<[u8]>>,
    dat_file: Option<Box<[u8]>>,
    cff_file: Option<Box<[u8]>>,
    encoding_label: Option<String>,
) -> Result<JsValue, JsValue> {
    let result = panic::catch_unwind(move || {
        if let Some(cff_data) = cff_file {
            // For CFF files, we assume UTF-8 as we can't easily decode only the text part
            // without a more sophisticated parsing approach.
            let cff_reader = BufReader::new(cff_data.as_ref());
            ComtradeParserBuilder::new()
                .cff_file(cff_reader)
                .build()
                .parse()
        } else if let (Some(cfg_data), Some(dat_data)) = (cfg_file, dat_file) {
            let encoding = encoding_label
                .as_deref()
                .and_then(|label| encoding_rs::Encoding::for_label(label.as_bytes()))
                .unwrap_or(encoding_rs::UTF_8);

            let (decoded_cfg, _, _) = encoding.decode(&cfg_data);
            let cfg_reader = BufReader::new(decoded_cfg.as_bytes());
            let dat_reader = BufReader::new(dat_data.as_ref()); // DAT file is binary
            ComtradeParserBuilder::new()
                .cfg_file(cfg_reader)
                .dat_file(dat_reader)
                .build()
                .parse()
        } else {
            panic!("Invalid file combination: either a CFF file, or both a CFG and a DAT file must be provided.");
        }
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
        Err(e) => {
            let message = if let Some(s) = e.downcast_ref::<&'static str>() {
                *s
            } else if let Some(s) = e.downcast_ref::<String>() {
                s
            } else {
                "A panic occurred while parsing the COMTRADE file. This may be due to a malformed file."
            };
            Err(JsValue::from_str(message))
        }
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}