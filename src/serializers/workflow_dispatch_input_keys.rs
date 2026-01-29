use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::collections::HashMap;

fn value_to_pretty_string(value: &Value) -> String {
    match value {
        Value::Null => "".to_string(),
        Value::String(s) => s.clone(),
        // Numbers / bools / arrays / objects: keep JSON representation
        other => other.to_string(),
    }
}

pub fn deserialize_input_pairs<'de, D>(deserializer: D) -> Result<Vec<(String, String)>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<HashMap<String, Value>>::deserialize(deserializer)?;
    Ok(opt
        .map(|map| {
            let mut pairs = map
                .into_iter()
                .map(|(k, v)| (k, value_to_pretty_string(&v)))
                .collect::<Vec<_>>();
            // Deterministic ordering for stable messages/tests
            pairs.sort_by(|a, b| a.0.cmp(&b.0));
            pairs
        })
        .unwrap_or_else(Vec::new))
}
