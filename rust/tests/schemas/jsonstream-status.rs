use serde_json::json;
use serde_json::Value;

pub fn schema() -> Value {
    json!({
        "type": "Toolbox",
        "chain": Value::Number(serde_json::Number::from(0)),
        "chain_code": Value::String(String::new()),
        "chain_name": Value::String(String::new()),
        "entity": Value::String(String::new()),
        "latest_block_height": Value::Number(serde_json::Number::from(0)),
        "service":Value::String(String::new()),
        "status": Value::String(String::new()),
        "timestamp": Value::Number(serde_json::Number::from(0)),
    })
}
