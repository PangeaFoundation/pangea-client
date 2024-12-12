use serde_json::json;
use serde_json::Value;

pub fn schema() -> Value {
    json!({
        "chain": Value::Number(serde_json::Number::from(0)),
        "block_number": Value::String(String::new()),
        "hash": Value::String(String::new()),
        "parent_hash": Value::String(String::new()),
        "uncles_hash": Value::String(String::new()),
        "miner": Value::String(String::new()),
        "state_root": Value::String(String::new()),
        "transactions_root": Value::String(String::new()),
        "receipts_root": Value::String(String::new()),
        "gas_used": Value::String(String::new()),
        "gas_limit": Value::String(String::new()),
        "base_fee_per_gas": Value::String(String::new()),
        "extra_data": Value::String(String::new()),
        "logs_bloom": Value::String(String::new()),
        "timestamp": Value::Number(serde_json::Number::from(0)),
        "difficulty": Value::String(String::new()),
        "total_difficulty": Value::String(String::new()),
        "size": Value::String(String::new()),
        "mix_hash": Value::String(String::new()),
        "nonce": Value::String(String::new())
    })
}
