// comtrade_rust/src/lib.rs
// This file is the main entry point for the Rust code that is compiled to WebAssembly.
// This file exists to parse COMTRADE files and return the information to the Svelte frontend.
// RELEVANT FILES: app/src/routes/info/+page.svelte

use comtrade::{ComtradeParserBuilder, DataFormat, StatusChannel};
use encoding_rs;
use regex::bytes::Regex as BytesRegex;
use serde::Serialize;
use std::{io::BufReader, panic};
use wasm_bindgen::prelude::*;

pub const GIT_HASH: &str = env!("GIT_HASH");

#[derive(Debug, thiserror::Error)]
pub enum WasmComtradeError {
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Invalid file combination: {0}")]
    InvalidFileCombination(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Internal panic: {0}")]
    PanicError(String),
}

impl From<WasmComtradeError> for JsValue {
    fn from(wasm: WasmComtradeError) -> Self {
        let err = js_sys::Error::new(&wasm.to_string());
        err.set_name("WasmComtradeError");
        err.into()
    }
}

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

/// Represents a single digital channel from a COMTRADE file, formatted for serialization.
#[derive(Serialize, Clone)]
pub struct SerializableDigitalChannel {
    /// The channel index number.
    pub index: u32,
    /// The name of the digital channel.
    pub name: String,
    /// The initial value of the channel.
    pub initial_value: u8,
}

impl From<&StatusChannel> for SerializableDigitalChannel {
    fn from(channel: &StatusChannel) -> Self {
        Self {
            index: channel.config.index.get() as u32,
            name: channel.config.name.clone(),
            initial_value: channel.config.normal_status_value,
        }
    }
}

/// Contains the parsed information from a COMTRADE file.
#[derive(Serialize)]
pub struct ComtradeInfo {
    /// The name of the substation or station where the recording was made.
    pub station: String,

    /// The identifier of the recording device (recorder ID).
    pub recording_device_id: String,

    /// Human-readable start timestamp of the recording as produced by the
    /// underlying COMTRADE library. Typically an ISO-like string.
    pub start_time: String,

    /// Human-readable trigger timestamp of the recorded event. Useful for UI
    /// display and understanding the event relative to the recording.
    pub trigger_time: String,

    /// The data format of the DAT file (e.g., "ASCII", "BINARY").
    pub data_format: String,

    /// The nominal line frequency in Hz (e.g., 50.0 or 60.0).
    pub frequency: f64,

    /// A list of the analog channels present in the file, with metadata and
    /// sample values for each channel.
    pub analog_channels: Vec<SerializableAnalogChannel>,

    /// A list of the digital/status channels present in the file. Each entry
    /// contains basic metadata (index, name, initial value).
    pub digital_channels: Vec<SerializableDigitalChannel>,

    /// Absolute timestamps for every sample expressed as Unix seconds (floating
    /// point). The length of this vector matches the number of samples in the
    /// recording and aligns with the per-channel sample arrays.
    pub timestamps: Vec<f64>,

    /// Non-fatal warnings encountered during parsing or analysis. These are
    /// intended to surface issues the user should be aware of (e.g. unexpected
    /// line frequency, missing trip signal after a detected sag) but which do
    /// not prevent returning a usable analysis.
    pub warnings: Vec<String>,

    /// Parsing or validation errors detected that indicate structural problems
    /// with the input file or clear mismatches (for example: declared total
    /// channel count not matching parsed channels). Presence of items here
    /// indicates the file may not be fully trustworthy or correctly formatted.
    pub errors: Vec<String>,

    /// Notes produced by automated analysis routines. These are informational
    /// observations such as "possible voltage sag detected" or "relay trip
    /// signal detected" and are intended to help an engineer quickly review
    /// noteworthy events in the recording.
    pub analysis_notes: Vec<String>,

    /// Numeric trigger timestamp as Unix seconds (floating point). This is
    /// provided as a machine-friendly numeric value useful for programmatic
    /// timing calculations and alignment.
    pub trigger_timestamp: f64,
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
) -> Result<JsValue, WasmComtradeError> {
    let encoding = encoding_label
        .as_deref()
        .and_then(|label| encoding_rs::Encoding::for_label(label.as_bytes()))
        .unwrap_or(encoding_rs::UTF_8);

    let result = panic::catch_unwind(move || {
        if let Some(cff_data) = cff_file {
            // CFF files can contain binary data (DAT part), so we split it into components first.
            // Some parsers fail if they try to read the entire file as UTF-8.
            let re =
                BytesRegex::new(r"(?im-u)^---\s*file type:\s*(?P<file_type>[a-z]+).*?---").unwrap();

            let mut cfg_raw = None;
            let mut dat_raw = None;
            let mut hdr_raw = None;
            let mut inf_raw = None;

            let matches: Vec<_> = re.find_iter(&cff_data).collect();
            for i in 0..matches.len() {
                let m = matches[i];
                let header = &cff_data[m.start()..m.end()];
                let caps = re.captures(header).unwrap();
                let file_type = std::str::from_utf8(&caps["file_type"])
                    .unwrap()
                    .to_lowercase();

                let next_start = if i + 1 < matches.len() {
                    matches[i + 1].start()
                } else {
                    cff_data.len()
                };

                let content = &cff_data[m.end()..next_start];

                match file_type.as_str() {
                    "cfg" => cfg_raw = Some(content),
                    "dat" => dat_raw = Some(content),
                    "hdr" => hdr_raw = Some(content),
                    "inf" => inf_raw = Some(content),
                    _ => {}
                }
            }

            let decoded_cfg = cfg_raw.map(|b| {
                let s = encoding.decode(b).0;
                let mut lines: Vec<_> = s.lines().filter(|l| !l.trim().is_empty()).collect();
                lines.push(""); // Add trailing blank line
                lines.join("\r\n")
            });
            let decoded_hdr = hdr_raw.map(|b| encoding.decode(b).0);
            let decoded_inf = inf_raw.map(|b| encoding.decode(b).0);

            if let Some(cfg) = &decoded_cfg {
                let mut builder = ComtradeParserBuilder::new();
                builder = builder.cfg_file(BufReader::new(cfg.as_bytes()));

                if let Some(dat_bytes) = dat_raw {
                    builder = builder.dat_file(BufReader::new(dat_bytes));
                }

                if let Some(hdr) = &decoded_hdr {
                    builder = builder.hdr_file(BufReader::new(hdr.as_bytes()));
                }

                if let Some(inf) = &decoded_inf {
                    builder = builder.inf_file(BufReader::new(inf.as_bytes()));
                }

                builder.build().parse()
            } else {
                panic!("No CFG section found in CFF file.");
            }
        } else if let (Some(cfg_data), Some(dat_data)) = (cfg_file, dat_file) {
            let (decoded_cfg, _, _) = encoding.decode(&cfg_data);
            let cfg_reader = BufReader::new(decoded_cfg.as_bytes());
            let dat_reader = BufReader::new(dat_data.as_ref()); // DAT file is binary
            ComtradeParserBuilder::new()
                .cfg_file(cfg_reader)
                .dat_file(dat_reader)
                .build()
                .parse()
        } else {
            panic!(
                "Invalid file combination: either a CFF file, or both a CFG and a DAT file must be provided."
            );
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
                    index: ch.config.index.get() as u32,
                    name: ch.config.name.clone(),
                    units: ch.config.units.clone(),
                    min_value: ch.config.min_value,
                    max_value: ch.config.max_value,
                    multiplier: ch.config.multiplier,
                    offset_adder: ch.config.offset_adder,
                    phase: ch.config.phase.clone(),
                    circuit_component_being_monitored: ch
                        .config
                        .circuit_component_being_monitored
                        .clone(),
                    values: ch.data.clone(),
                })
                .collect();

            let digital_channels: Vec<SerializableDigitalChannel> = comtrade
                .status_channels
                .iter()
                .map(SerializableDigitalChannel::from)
                .collect();

            let mut warnings = Vec::new();
            let mut errors = Vec::new();
            let mut analysis_notes = Vec::new();

            let actual_total_channels =
                comtrade.analog_channels.len() + comtrade.status_channels.len();
            if comtrade.declared_total_channels != actual_total_channels {
                errors.push(format!(
                    "The total number of channels ({}) does not match the sum of analog ({}) and digital ({}) channels.",
                    comtrade.declared_total_channels,
                    comtrade.analog_channels.len(),
                    comtrade.status_channels.len()
                ));
            }

            // Check frequency
            if comtrade.line_frequency != 0.0
                && (comtrade.line_frequency - 50.0).abs() > 1.0
                && (comtrade.line_frequency - 60.0).abs() > 1.0
            {
                warnings.push(format!(
                    "Unexpected frequency detected ({} Hz).",
                    comtrade.line_frequency
                ));
            }

            // Detect voltage sag
            let mut sag_detected = false;
            let mut sag_start_time = 0.0;

            for channel in &analog_channels {
                let name = channel.name.to_lowercase();
                let units = channel.units.to_lowercase();
                if name.contains("v") || units == "v" || units == "kv" {
                    let window_size = 50.min(channel.values.len());
                    if window_size > 0 {
                        let mut sum_sq_init = 0.0;
                        for i in 0..window_size {
                            sum_sq_init += channel.values[i] * channel.values[i];
                        }
                        let nominal_rms = (sum_sq_init / window_size as f64).sqrt();

                        let sag_threshold = nominal_rms * 0.8;
                        let mut sum_sq = 0.0;
                        for i in 0..window_size {
                            sum_sq += channel.values[i] * channel.values[i];
                        }

                        let mut rms = (sum_sq / window_size as f64).sqrt();
                        if rms < sag_threshold {
                            sag_detected = true;
                            sag_start_time = absolute_timestamps[window_size - 1];
                            analysis_notes.push(format!(
                                "Possible voltage sag detected on channel '{}' at {:.4} seconds.",
                                channel.name, sag_start_time
                            ));
                            break;
                        }

                        for i in window_size..channel.values.len() {
                            sum_sq += channel.values[i] * channel.values[i];
                            sum_sq -=
                                channel.values[i - window_size] * channel.values[i - window_size];
                            if sum_sq < 0.0 {
                                sum_sq = 0.0;
                            }
                            rms = (sum_sq / window_size as f64).sqrt();
                            if rms < sag_threshold {
                                sag_detected = true;
                                sag_start_time = absolute_timestamps[i];
                                analysis_notes.push(format!("Possible voltage sag detected on channel '{}' at {:.4} seconds.", channel.name, sag_start_time));
                                break;
                            }
                        }
                        if sag_detected {
                            break;
                        }
                    }
                }
            }

            if sag_detected {
                let mut trip_found = false;
                for (digital_index, digital_channel) in digital_channels.iter().enumerate() {
                    let name = digital_channel.name.to_lowercase();
                    if name.contains("trip") {
                        let Some(states) = comtrade.digital(digital_index) else {
                            continue;
                        };

                        for (j, &val) in states.iter().enumerate() {
                            if val == 1 {
                                let trip_time = absolute_timestamps[j];
                                if trip_time > sag_start_time {
                                    trip_found = true;
                                    let trip_delay = trip_time - sag_start_time;
                                    analysis_notes.push(format!(
                                        "Relay trip signal detected at {:.4} seconds.",
                                        trip_time
                                    ));
                                    analysis_notes.push(format!("Trip delay: {:.2} ms. Check if this is within acceptable limits.", trip_delay * 1000.0));
                                    break;
                                }
                            }
                        }
                        if trip_found {
                            break;
                        }
                    }
                }

                if !trip_found {
                    warnings.push("No trip signal detected after the voltage sag.".to_string());
                }
            }

            let info = ComtradeInfo {
                station: comtrade.station_name.clone(),
                recording_device_id: comtrade.recording_device_id.clone(),
                start_time: comtrade.start_time.to_string(),
                trigger_time: comtrade.trigger_time.to_string(),
                data_format: data_format_to_str(&comtrade.data_format).to_string(),
                frequency: comtrade.line_frequency,
                analog_channels,
                digital_channels,
                timestamps: absolute_timestamps,
                warnings,
                errors,
                analysis_notes,
                trigger_timestamp: trigger_time_seconds as f64,
            };
            serde_wasm_bindgen::to_value(&info)
                .map_err(|e| WasmComtradeError::SerializationError(e.to_string()))
        }
        Ok(Err(e)) => Err(WasmComtradeError::ParseError(format!("{:?}", e))),
        Err(e) => {
            let message = if let Some(s) = e.downcast_ref::<&'static str>() {
                *s
            } else if let Some(s) = e.downcast_ref::<String>() {
                s
            } else {
                "A panic occurred while parsing the COMTRADE file. This may be due to a malformed file."
            };
            Err(WasmComtradeError::PanicError(message.to_string()))
        }
    }
}

/// Sets up a panic hook and logs the build info on WASM load
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let version = env!("CARGO_PKG_VERSION");
    let msg = format!("comtrade_rust v{} ({})", version, GIT_HASH);
    let js_msg = JsValue::from_str(&msg);
    web_sys::console::info_1(&js_msg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_cff() {
        let cff_data = b"--- file type: CFG ---\nCFG CONTENT\n--- file type: DAT ---\nDAT CONTENT\n--- file type: INF ---\nINF CONTENT\n--- file type: HDR ---\nHDR CONTENT";
        let re =
            BytesRegex::new(r"(?im-u)^---\s*file type:\s*(?P<file_type>[a-z]+).*?---").unwrap();

        let mut cfg_raw = None;
        let mut dat_raw = None;
        let mut hdr_raw = None;
        let mut inf_raw = None;

        let matches: Vec<_> = re.find_iter(cff_data).collect();
        for i in 0..matches.len() {
            let m = matches[i];
            let header = &cff_data[m.start()..m.end()];
            let caps = re.captures(header).unwrap();
            let file_type = std::str::from_utf8(&caps["file_type"])
                .unwrap()
                .to_lowercase();

            let next_start = if i + 1 < matches.len() {
                matches[i + 1].start()
            } else {
                cff_data.len()
            };

            let content = &cff_data[m.end()..next_start];

            match file_type.as_str() {
                "cfg" => cfg_raw = Some(content),
                "dat" => dat_raw = Some(content),
                "hdr" => hdr_raw = Some(content),
                "inf" => inf_raw = Some(content),
                _ => {}
            }
        }

        assert_eq!(
            std::str::from_utf8(cfg_raw.unwrap()).unwrap().trim(),
            "CFG CONTENT"
        );
        assert_eq!(
            std::str::from_utf8(dat_raw.unwrap()).unwrap().trim(),
            "DAT CONTENT"
        );
        assert_eq!(
            std::str::from_utf8(inf_raw.unwrap()).unwrap().trim(),
            "INF CONTENT"
        );
        assert_eq!(
            std::str::from_utf8(hdr_raw.unwrap()).unwrap().trim(),
            "HDR CONTENT"
        );
    }
}
