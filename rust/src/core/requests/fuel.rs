use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    core::types::{
        fuel::{
            LimitType, MarketEventType, MessageType, OrderEventType, OrderType, ReceiptType,
            TransactionType,
        },
        ChainId,
    },
    query::Bound,
    utils::serialize_comma_separated,
};

use fuel_core_types::fuel_types::Address;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetFuelBlocksRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Exclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub da_block_number__gte: Option<u64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub da_block_number__lte: Option<u64>,
}

impl Default for GetFuelBlocksRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            da_block_number__gte: None,
            da_block_number__lte: None,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetFuelLogsRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Exclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub id__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub ra__in: HashSet<u64>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub rb__in: HashSet<u64>,
}

impl Default for GetFuelLogsRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            id__in: HashSet::new(),
            ra__in: HashSet::new(),
            rb__in: HashSet::new(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetFuelTxsRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Exclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub transaction_type__in: HashSet<TransactionType>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub metadata_contract_id__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub input_contract_contract_id__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub mint_asset_id__in: HashSet<Address>,

    #[serde(default)]
    pub mint_amount__lte: Option<u64>,

    #[serde(default)]
    pub mint_amount__gte: Option<u64>,
}

impl Default for GetFuelTxsRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            transaction_type__in: HashSet::new(),
            metadata_contract_id__in: HashSet::new(),
            input_contract_contract_id__in: HashSet::new(),
            mint_asset_id__in: HashSet::new(),
            mint_amount__lte: None,
            mint_amount__gte: None,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetFuelReceiptsRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Exclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub receipt_type__in: HashSet<ReceiptType>,
}

impl Default for GetFuelReceiptsRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            receipt_type__in: HashSet::new(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetFuelMessagesRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Exclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    pub da_block_number__gte: Option<u64>,

    #[serde(default)]
    pub da_block_number__lte: Option<u64>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub sender__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub recipient__in: HashSet<Address>,

    #[serde(default)]
    pub amount__gte: Option<u64>,

    #[serde(default)]
    pub amount__lte: Option<u64>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub message_type__in: HashSet<MessageType>,
}

impl Default for GetFuelMessagesRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            da_block_number__gte: None,
            da_block_number__lte: None,
            sender__in: HashSet::new(),
            recipient__in: HashSet::new(),
            amount__gte: None,
            amount__lte: None,
            message_type__in: HashSet::new(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetSparkMarketRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    pub from_block: Bound,
    // Inclusive upper bound if is Some for block number
    pub to_block: Bound,

    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub event_type__in: HashSet<MarketEventType>,
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub base_asset__in: HashSet<Address>,
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub quote_asset__in: HashSet<Address>,
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub market_id__in: HashSet<Address>,
}

impl Default for GetSparkMarketRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            event_type__in: HashSet::default(),
            base_asset__in: HashSet::default(),
            quote_asset__in: HashSet::default(),
            market_id__in: HashSet::default(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetSparkOrderRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Exclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub order_id__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub order_type__in: HashSet<OrderType>,
    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub event_type__in: HashSet<OrderEventType>,
    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub limit_type__in: HashSet<LimitType>,
    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub user__in: HashSet<Address>,
    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub asset__in: HashSet<Address>,
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub market_id__in: HashSet<Address>,
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub address__in: HashSet<Address>,
}

impl Default for GetSparkOrderRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            order_id__in: HashSet::new(),
            order_type__in: HashSet::new(),
            event_type__in: HashSet::new(),
            limit_type__in: HashSet::new(),
            user__in: HashSet::new(),
            asset__in: HashSet::new(),
            market_id__in: HashSet::new(),
            address__in: HashSet::new(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetUtxoRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default = "Bound::none")]
    pub from_block: Bound,
    // Exclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    pub unspent_at: Bound,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub address__in: HashSet<Address>,
}

impl Default for GetUtxoRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            unspent_at: Bound::default(),
            address__in: HashSet::new(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetSrc20 {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default = "default_src20_from_block")]
    pub from_block: Bound,
    // Exclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub contract_id__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub asset_id__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub symbol__in: HashSet<String>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub name__in: HashSet<String>,

    #[serde(default)]
    pub decimals__gte: Option<u8>,

    #[serde(default)]
    pub decimals__lte: Option<u8>,
}

impl Default for GetSrc20 {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: default_src20_from_block(),
            to_block: Bound::default(),
            contract_id__in: HashSet::new(),
            asset_id__in: HashSet::new(),
            symbol__in: HashSet::new(),
            name__in: HashSet::new(),
            decimals__gte: None,
            decimals__lte: None,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetSrc7 {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default = "default_src20_from_block")]
    pub from_block: Bound,
    // Exclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub asset__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub key__in: HashSet<String>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub sender__in: HashSet<Address>,
}

impl Default for GetSrc7 {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: default_src20_from_block(),
            to_block: Bound::default(),
            asset__in: HashSet::default(),
            key__in: HashSet::default(),
            sender__in: HashSet::default(),
        }
    }
}

fn default_src20_from_block() -> Bound {
    Bound::Exact(0)
}

pub fn default_chains() -> HashSet<ChainId> {
    HashSet::from([ChainId::FUEL])
}
