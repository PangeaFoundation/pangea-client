use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};

use super::ChainId;

#[derive(
    AsRefStr, EnumString, Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Default,
)]
pub enum Type {
    #[default]
    #[strum(to_string = "N/A")]
    Unknown,
    #[strum(to_string = "CHAIN")]
    Chain,
    #[strum(to_string = "TOOLBOX")]
    Toolbox,
}

#[derive(
    AsRefStr, EnumString, Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Default,
)]
pub enum HealthStatus {
    #[default]
    #[strum(to_string = "OK")]
    Ok,
    #[strum(to_string = "Down")]
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Default)]
pub struct Status {
    pub r#type: Type,
    pub chain: ChainId,
    pub chain_code: String,
    pub chain_name: String,
    pub service: String,
    pub entity: String,
    pub latest_block_height: u64,
    pub timestamp: u64,
    pub status: HealthStatus,
}
