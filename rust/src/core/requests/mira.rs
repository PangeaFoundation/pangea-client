use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use ethers_core::types::H256;

use crate::{core::types::ChainId, query::Bound, utils::serialize_comma_separated};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetMiraPoolsRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Inclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub pool_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "asset0__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub asset0_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "asset1__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub asset1_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub assets__in: HashSet<H256>,
}

impl Default for GetMiraPoolsRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            pool_address__in: HashSet::default(),
            asset0_address__in: HashSet::default(),
            asset1_address__in: HashSet::default(),
            assets__in: HashSet::default(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetMiraLiquidityRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Inclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub pool_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "asset0__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub asset0_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "asset1__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub asset1_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub assets__in: HashSet<H256>,
}

impl Default for GetMiraLiquidityRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            pool_address__in: HashSet::default(),
            asset0_address__in: HashSet::default(),
            asset1_address__in: HashSet::default(),
            assets__in: HashSet::default(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetMiraSwapsRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Inclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub pool_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "asset0__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub asset0_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "asset1__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub asset1_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub assets__in: HashSet<H256>,
}

impl Default for GetMiraSwapsRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            pool_address__in: HashSet::default(),
            asset0_address__in: HashSet::default(),
            asset1_address__in: HashSet::default(),
            assets__in: HashSet::default(),
        }
    }
}

fn default_chains() -> HashSet<ChainId> {
    HashSet::from([ChainId::FUEL])
}
