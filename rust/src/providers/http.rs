use std::{collections::HashSet, str::FromStr};

use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use futures::{StreamExt, TryStreamExt};
use reqwest::header;

use crate::requests::arche::{GetCollateralsRequest, GetLoansRequest, GetPositionsRequest};
use crate::requests::movement::GetBalancesRequest;
use crate::requests::pyth;
use crate::{
    core::{
        error::{Error, Result},
        types::format::Format,
    },
    provider::{
        BtcProvider, ChainProvider, CurveProvider, Erc20Provider, FuelProvider, MoveProvider,
        Provider, StreamResponse, UniswapV2Provider, UniswapV3Provider,
    },
    requests::{
        blocks, btc, curve, erc20, fuel, interest, logs, mira, movement, transfers, txs,
        uniswap_v2, uniswap_v3,
    },
    ChainId,
};

const API_PATH: &str = "v1/api/";

pub struct HttpProvider {
    inner: reqwest::Client,
    base_url: reqwest::Url,
}

impl HttpProvider {
    async fn request<R>(
        &self,
        url: reqwest::Url,
        request: R,
        format: Format,
    ) -> StreamResponse<Vec<u8>>
    where
        R: serde::Serialize,
    {
        let raw_data_stream = self
            .inner
            .get(url)
            .query(&request)
            .query(&[("format", format)])
            .send()
            .await?
            // .error_for_status()?
            .bytes_stream()
            .map_err(Error::from)
            .map_ok(|bytes| bytes.to_vec())
            .boxed();

        Ok(raw_data_stream)
    }

    fn url(&self, path: &str) -> Result<reqwest::Url> {
        self.base_url.join(path).map_err(Error::from)
    }
}

const STATUS_PATH: &str = "status";

#[async_trait]
impl Provider for HttpProvider {
    async fn try_new(
        endpoint: String,
        is_secure: bool,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let (Some(username), Some(password)) = (username, password) {
            let auth = format!("{username}:{password}");
            let encoded = BASE64.encode(auth);
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Basic {encoded}"))
                    .expect("Only non-ascii chars result in an error"),
            );
        }

        let base_url = reqwest::Url::from_str(&format!(
            "{}://{endpoint}/{API_PATH}",
            if is_secure { "https" } else { "http" }
        ))?;

        let inner = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .https_only(is_secure)
            .build()?;

        Ok(Self { inner, base_url })
    }

    async fn get_status_by_format(&self, format: Format) -> StreamResponse<Vec<u8>> {
        let url = self.url(STATUS_PATH)?;
        self.request(url, (), format).await
    }
}

const ETHEREUM_BLOCKS_PATH: &str = "blocks";
const ETHEREUM_LOGS_PATH: &str = "logs";
const ETHEREUM_TRANSACTIONS_PATH: &str = "transactions";
const ETHEREUM_TRANSFERS_PATH: &str = "transfers";

#[async_trait]
impl ChainProvider for HttpProvider {
    async fn get_blocks_by_format(
        &self,
        request: blocks::GetBlocksRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(ETHEREUM_BLOCKS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_logs_by_format(
        &self,
        request: logs::GetLogsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(ETHEREUM_LOGS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_txs_by_format(
        &self,
        request: txs::GetTxsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(ETHEREUM_TRANSACTIONS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_transfers_by_format(
        &self,
        request: transfers::GetTransfersRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(ETHEREUM_TRANSFERS_PATH)?;
        self.request(url, request, format).await
    }
}

const UNISWAP_V2_PAIRS_PATH: &str = "uniswap/v2/pairs";
const UNISWAP_V2_PRICES_PATH: &str = "uniswap/v2/prices";

#[async_trait]
impl UniswapV2Provider for HttpProvider {
    async fn get_pairs_by_format(
        &self,
        request: uniswap_v2::GetPairsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(UNISWAP_V2_PAIRS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_prices_by_format(
        &self,
        request: uniswap_v2::GetPricesRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(UNISWAP_V2_PRICES_PATH)?;
        self.request(url, request, format).await
    }
}

const UNISWAP_V3_FEES_PATH: &str = "uniswap/v3/fees";
const UNISWAP_V3_POOLS_PATH: &str = "uniswap/v3/pools";
const UNISWAP_V3_POSITIONS: &str = "uniswap/v3/positions";
const UNISWAP_V3_PRICES_PATH: &str = "uniswap/v3/prices";

#[async_trait]
impl UniswapV3Provider for HttpProvider {
    async fn get_fees_by_format(
        &self,
        request: uniswap_v3::GetFeesRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(UNISWAP_V3_FEES_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_pools_by_format(
        &self,
        request: uniswap_v3::GetPoolsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(UNISWAP_V3_POOLS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_prices_by_format(
        &self,
        request: uniswap_v3::GetPricesRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(UNISWAP_V3_PRICES_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_positions_by_format(
        &self,
        request: uniswap_v3::GetPositionsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(UNISWAP_V3_POSITIONS)?;
        self.request(url, request, format).await
    }
}

const CURVE_TOKENS_PATH: &str = "curve/tokens";
const CURVE_POOLS_PATH: &str = "curve/pools";
const CURVE_PRICES_PATH: &str = "curve/prices";

#[async_trait]
impl CurveProvider for HttpProvider {
    async fn get_tokens_by_format(
        &self,
        request: curve::GetCrvTokenRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(CURVE_TOKENS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_pools_by_format(
        &self,
        request: curve::GetCrvPoolRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(CURVE_POOLS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_prices_by_format(
        &self,
        request: curve::GetCrvPriceRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(CURVE_PRICES_PATH)?;
        self.request(url, request, format).await
    }
}

const ERC20_TOKENS_PATH: &str = "erc20";
const ERC20_APPROVALS_PATH: &str = "erc20/approvals";
const ERC20_TRANSFERS_PATH: &str = "erc20/transfers";

#[async_trait]
impl Erc20Provider for HttpProvider {
    async fn get_erc20_by_format(
        &self,
        request: erc20::GetErc20Request,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(ERC20_TOKENS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_erc20_approval_by_format(
        &self,
        request: erc20::GetErc20ApprovalsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(ERC20_APPROVALS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_erc20_transfers_by_format(
        &self,
        request: erc20::GetErc20TransferssRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(ERC20_TRANSFERS_PATH)?;
        self.request(url, request, format).await
    }
}

const FUEL_BLOCKS_PATH: &str = "blocks";
const FUEL_LOGS_PATH: &str = "logs";
const FUEL_LOGS_DECODED_PATH: &str = "logs/decoded";
const FUEL_TRANSACTIONS_PATH: &str = "transactions";
const FUEL_UNSPENT_UTXOS_PATH: &str = "transactions/outputs";
const FUEL_RECEIPTS_PATH: &str = "receipts";
const FUEL_MESSAGES_PATH: &str = "messages";
const FUEL_SPARK_MARKET_PATH: &str = "spark/markets";
const FUEL_SPARK_ORDER_PATH: &str = "spark/orders";
const FUEL_SRC20_PATH: &str = "src20";
const FUEL_SRC7_PATH: &str = "src7";
const FUEL_MIRA_POOLS_PATH: &str = "mira/v1/pools";
const FUEL_MIRA_LIQUIDITY_PATH: &str = "mira/v1/liquidity";
const FUEL_MIRA_SWAPS_PATH: &str = "mira/v1/swaps";

#[async_trait]
impl FuelProvider for HttpProvider {
    async fn get_fuel_blocks_by_format(
        &self,
        request: fuel::GetFuelBlocksRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(FUEL_BLOCKS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_logs_by_format(
        &self,
        request: fuel::GetFuelLogsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(FUEL_LOGS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_logs_decoded_by_format(
        &self,
        request: fuel::GetFuelLogsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(FUEL_LOGS_DECODED_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_txs_by_format(
        &self,
        request: fuel::GetFuelTxsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(FUEL_TRANSACTIONS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_receipts_by_format(
        &self,
        request: fuel::GetFuelReceiptsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(FUEL_RECEIPTS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_messages_by_format(
        &self,
        request: fuel::GetFuelMessagesRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(FUEL_MESSAGES_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_unspent_utxos_by_format(
        &self,
        request: fuel::GetUtxoRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(FUEL_UNSPENT_UTXOS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_spark_markets_by_format(
        &self,
        request: fuel::GetSparkMarketRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(FUEL_SPARK_MARKET_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_spark_orders_by_format(
        &self,
        request: fuel::GetSparkOrderRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(FUEL_SPARK_ORDER_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_src20_by_format(
        &self,
        request: fuel::GetSrc20,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(FUEL_SRC20_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_src7_by_format(
        &self,
        request: fuel::GetSrc7,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(FUEL_SRC7_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_mira_v1_pools_by_format(
        &self,
        request: mira::GetMiraPoolsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(FUEL_MIRA_POOLS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_mira_v1_liquidity_by_format(
        &self,
        request: mira::GetMiraLiquidityRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(FUEL_MIRA_LIQUIDITY_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_mira_v1_swaps_by_format(
        &self,
        request: mira::GetMiraSwapsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(FUEL_MIRA_SWAPS_PATH)?;
        self.request(url, request, format).await
    }
}

const MOVE_LOGS_PATH: &str = "logs";
const MOVE_LOGS_DECODED_PATH: &str = "logs/decoded";
const MOVE_TRANSACTIONS_PATH: &str = "transactions";
const MOVE_TRANSACTIONS_DECODED_PATH: &str = "transactions/decoded";
const MOVE_RECEIPTS_PATH: &str = "receipts";
const MOVE_RECEIPTS_DECODED_PATH: &str = "receipts/decoded";
const MOVE_MODULES_PATH: &str = "modules";
const MOVE_FA_TOKENS_PATH: &str = "fa-tokens";
const MOVE_INTEREST_V1_POOLS_PATH: &str = "interest/v1/pools";
const MOVE_INTEREST_V1_LIQUIDITY_PATH: &str = "interest/v1/liquidity";
const MOVE_INTEREST_V1_SWAPS_PATH: &str = "interest/v1/swaps";
const MOVE_ARCHE_COLLATERALS_PATH: &str = "arche/collaterals";
const MOVE_ARCHE_LOANS_PATH: &str = "arche/loans";
const MOVE_ARCHE_POSITIONS_PATH: &str = "arche/positions";
const MOVE_PYTH_PATH: &str = "pyth";
const MOVE_BALANCES_PATH: &str = "balances";
#[async_trait]
impl MoveProvider for HttpProvider {
    async fn get_move_logs_by_format(
        &self,
        request: movement::GetMoveLogsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_LOGS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_move_logs_decoded_by_format(
        &self,
        request: movement::GetMoveLogsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_LOGS_DECODED_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_move_txs_by_format(
        &self,
        request: movement::GetMoveTxsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_TRANSACTIONS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_move_txs_decoded_by_format(
        &self,
        request: movement::GetMoveTxsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_TRANSACTIONS_DECODED_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_move_receipts_by_format(
        &self,
        request: movement::GetMoveReceiptsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_RECEIPTS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_move_receipts_decoded_by_format(
        &self,
        request: movement::GetMoveReceiptsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_RECEIPTS_DECODED_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_move_modules_by_format(
        &self,
        request: movement::GetMoveReceiptsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_MODULES_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_move_fa_tokens_by_format(
        &self,
        request: movement::GetTokensRequest,
        format: Format,
        _deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_FA_TOKENS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_move_interest_v1_pools_by_format(
        &self,
        request: interest::GetPoolsRequest,
        format: Format,
        _deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_INTEREST_V1_POOLS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_move_interest_v1_liquidity_by_format(
        &self,
        request: interest::GetLiquidityRequest,
        format: Format,
        _deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_INTEREST_V1_LIQUIDITY_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_move_interest_v1_swaps_by_format(
        &self,
        request: interest::GetSwapsRequest,
        format: Format,
        _deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_INTEREST_V1_SWAPS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_move_arche_collaterals_by_format(
        &self,
        request: GetCollateralsRequest,
        format: Format,
        _deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_ARCHE_COLLATERALS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_move_arche_loans_by_format(
        &self,
        request: GetLoansRequest,
        format: Format,
        _deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_ARCHE_LOANS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_move_arche_positions_by_format(
        &self,
        request: GetPositionsRequest,
        format: Format,
        _deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_ARCHE_POSITIONS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_move_pyth_by_format(
        &self,
        request: pyth::GetPricesRequest,
        format: Format,
        _deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_PYTH_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_move_balances_by_format(
        &self,
        request: GetBalancesRequest,
        format: Format,
        _deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(MOVE_BALANCES_PATH)?;
        self.request(url, request, format).await
    }
}

const BTC_BLOCKS_PATH: &str = "blocks";
const BTC_TRANSACTIONS_PATH: &str = "transactions";
#[async_trait]
impl BtcProvider for HttpProvider {
    async fn get_btc_blocks_by_format(
        &self,
        mut request: btc::GetBtcBlocksRequest,
        format: Format,
        _deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        request.chains = HashSet::from_iter(vec![ChainId::BTC]);
        let url = self.url(BTC_BLOCKS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_btc_txs_by_format(
        &self,
        mut request: btc::GetBtcTxsRequest,
        format: Format,
        _deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        request.chains = HashSet::from_iter(vec![ChainId::BTC]);
        let url = self.url(BTC_TRANSACTIONS_PATH)?;
        self.request(url, request, format).await
    }
}
