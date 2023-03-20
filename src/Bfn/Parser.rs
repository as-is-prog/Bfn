use serde::{Deserialize, Serialize};
use serde_json;

pub fn ParseJson(json: &str) -> Result<serde_json::Value, serde_json::Error> {
    let v: serde_json::Value = serde_json::from_str(json)?;

    Ok(v)
}
