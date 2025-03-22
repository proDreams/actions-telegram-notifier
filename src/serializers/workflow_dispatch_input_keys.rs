use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::collections::HashMap;

pub fn deserialize_input_keys<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<HashMap<String, Value>>::deserialize(deserializer)?;
    Ok(opt
        .map(|map| map.into_keys().collect())
        .unwrap_or_else(Vec::new))
}
