// comtrade_rust/src/lib.rs
// This file is the main entry point for the Rust code that is compiled to WebAssembly.
// This file exists to parse COMTRADE files and return the information to the Svelte frontend.
// RELEVANT FILES: app/src/routes/info/+page.svelte

use comtrade::{ComtradeParserBuilder, DataFormat};
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

/// Represents a single analog channel from a COMTRADE file, formatted for serialization.
#[derive(Serialize, Clone)]
pub struct SerializableAnalogChannel {
    /// The channel index number.
    pub index: u32,
    /// The name of the analog channel.
    pub name: String,
    /// The units of measurement for the channel (e.g., "V", "A").
    pub units: String,
    /// The minimum value recorded for this channel.
    pub min_value: f64,
    /// The maximum value recorded for this channel.
    pub max_value: f64,
    /// The multiplier to apply to the channel's data values.
    pub multiplier: f64,
    /// The offset to add to the channel's data values.
    pub offset_adder: f64,
    /// The phase of the channel (e.g., "A", "B", "C").
    pub phase: String,
    /// The component of the power system circuit being monitored.
    pub circuit_component_being_monitored: String,
    /// The waveform data for this channel.
    pub values: Vec<f64>,
}

/// Contains the parsed information from a COMTRADE file.
#[derive(Serialize)]
pub struct ComtradeInfo {
    /// The name of the substation or station.
    pub station: String,
    /// The identifier of the recording device.
    pub recording_device_id: String,
    /// The start timestamp of the recording.
    pub start_time: String,
    /// The trigger timestamp of the event.
    pub trigger_time: String,
    /// The data format of the DAT file (e.g., "ASCII", "BINARY").
    pub data_format: String,
    /// The nominal line frequency in Hz.
    pub frequency: f64,
    /// A list of the analog channels present in the file.
    pub analog_channels: Vec<SerializableAnalogChannel>,
    /// The timestamps for each data point, in Unix seconds.
    pub timestamps: Vec<f64>,
}

/// Parses a COMTRADE file from its constituent parts.
///
/// Accepts either a single CFF file, or a pair of CFG and DAT files.
///
/// # Arguments
///
/// * `cfg_file` - An optional byte array of the .cfg file content.
/// * `dat_file` - An optional byte array of the .dat file content.
/// * `cff_file` - An optional byte array of the .cff file content.
/// * `encoding_label` - An optional string label for the text encoding of the CFG file (e.g., "utf-8", "latin1").
///                      Defaults to UTF-8 if not provided. This is ignored for CFF files.
///
/// # Returns
///
/// A `JsValue` containing the serialized `ComtradeInfo` on success, or a `JsValue` with an error message on failure.
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
            let trigger_time_seconds = comtrade.trigger_time.and_utc().timestamp();

            let mut timestamps_us = Vec::new();
            let mut current_time_us = 0.0;
            let mut last_end_sample = 0;

            for rate_info in &comtrade.sampling_rates {
                let period_us = 1_000_000.0 / rate_info.rate_hz as f64;
                let num_samples_in_section = rate_info.end_sample_number - last_end_sample;

                for _ in 0..num_samples_in_section {
                    timestamps_us.push(current_time_us);
                    current_time_us += period_us;
                }
                last_end_sample = rate_info.end_sample_number;
            }

            let absolute_timestamps: Vec<f64> = timestamps_us
                .iter()
                .map(|&t_us| trigger_time_seconds as f64 + (t_us / 1_000_000.0))
                .collect();

            let analog_channels: Vec<SerializableAnalogChannel> = comtrade
                .analog_channels
                .iter()
                .map(|ch| SerializableAnalogChannel {
                    index: ch.index,
                    name: ch.name.clone(),
                    units: ch.units.clone(),
                    min_value: ch.min_value,
                    max_value: ch.max_value,
                    multiplier: ch.multiplier,
                    offset_adder: ch.offset_adder,
                    phase: ch.phase.clone(),
                    circuit_component_being_monitored: ch.circuit_component_being_monitored.clone(),
                    values: ch.data.clone(),
                })
                .collect();

            let info = ComtradeInfo {
                station: comtrade.station_name.clone(),
                recording_device_id: comtrade.recording_device_id.clone(),
                start_time: comtrade.start_time.to_string(),
                trigger_time: comtrade.trigger_time.to_string(),
                data_format: data_format_to_str(&comtrade.data_format).to_string(),
                frequency: comtrade.line_frequency,
                analog_channels,
                timestamps: absolute_timestamps,
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

/// Sets up a panic hook to log panic messages to the browser's developer console.
/// This should be called once when the Wasm module is initialized.
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
