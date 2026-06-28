use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Deserialize)]
pub struct WasmFormatOptions {
    pub group_threshold: usize,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct WasmFormatResult {
    pub output: String,
}

#[wasm_bindgen]
pub fn format_xaml(input: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let options: WasmFormatOptions = serde_wasm_bindgen::from_value(options)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let output = xaml_lang_formatter::api::format_xaml(
        input,
        &xaml_lang_formatter::api::FormatOptions {
            group_threshold: options.group_threshold,
            timestamp: options.timestamp,
        },
    )
    .map_err(|err| JsValue::from_str(&err.to_string()))?;

    serde_wasm_bindgen::to_value(&WasmFormatResult { output })
        .map_err(|err| JsValue::from_str(&err.to_string()))
}
