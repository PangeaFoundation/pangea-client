use async_trait::async_trait;
use futures::StreamExt;

use super::{
    error::ResponseError,
    provider::{
        BtcProvider, ChainProvider, CurveProvider, Erc20Provider, FuelProvider, MoveProvider,
        Provider, StreamResponse, UniswapV2Provider, UniswapV3Provider,
    },
    requests::{
        blocks, btc, curve, erc20, fuel, interest, logs, mira, movement, transfers, txs,
        uniswap_v2, uniswap_v3,
    },
    types::{format::Format, status::Status},
};
use crate::requests::arche::{GetCollateralsRequest, GetLoansRequest, GetPositionsRequest};
use crate::requests::movement::GetBalancesRequest;
use crate::requests::pyth;
use crate::{Operation, WsProvider};

pub struct Client<T> {
    pub inner: T,
}

impl<T> Client<T>
where
    T: Provider,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub async fn get_status(&self) -> StreamResponse<Status> {
        let raw_data_stream = self.inner.get_status_by_format(Format::JsonStream).await?;
        let raw_data_stream = ResponseError::map_stream(raw_data_stream);

        let records = raw_data_stream
            .map(|chunk_result| {
                chunk_result.and_then(|chunk| Ok(serde_json::from_slice::<Status>(&chunk)?))
            })
            .boxed();

        Ok(records)
    }
}

impl Client<WsProvider> {
    pub async fn raw_request(
        &self,
        operation: Operation,
        params: impl serde::Serialize,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner.request(operation, params, format, deltas).await
    }
}

#[async_trait]
impl<T> ChainProvider for Client<T>
where
    T: ChainProvider + Send + Sync,
{
    async fn get_blocks_by_format(
        &self,
        request: blocks::GetBlocksRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_blocks_by_format(request, format, deltas)
            .await
    }

    async fn get_logs_by_format(
        &self,
        request: logs::GetLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner.get_logs_by_format(request, format, deltas).await
    }

    async fn get_txs_by_format(
        &self,
        request: txs::GetTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner.get_txs_by_format(request, format, deltas).await
    }

    async fn get_transfers_by_format(
        &self,
        request: transfers::GetTransfersRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_transfers_by_format(request, format, deltas)
            .await
    }
}

#[async_trait]
impl<T> UniswapV2Provider for Client<T>
where
    T: UniswapV2Provider + Send + Sync,
{
    async fn get_pairs_by_format(
        &self,
        request: uniswap_v2::GetPairsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_pairs_by_format(request, format, deltas)
            .await
    }

    async fn get_prices_by_format(
        &self,
        request: uniswap_v2::GetPricesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_prices_by_format(request, format, deltas)
            .await
    }
}

#[async_trait]
impl<T> UniswapV3Provider for Client<T>
where
    T: UniswapV3Provider + Send + Sync,
{
    async fn get_fees_by_format(
        &self,
        request: uniswap_v3::GetFeesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner.get_fees_by_format(request, format, deltas).await
    }

    async fn get_pools_by_format(
        &self,
        request: uniswap_v3::GetPoolsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_pools_by_format(request, format, deltas)
            .await
    }

    async fn get_prices_by_format(
        &self,
        request: uniswap_v3::GetPricesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_prices_by_format(request, format, deltas)
            .await
    }

    async fn get_positions_by_format(
        &self,
        request: uniswap_v3::GetPositionsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_positions_by_format(request, format, deltas)
            .await
    }
}

#[async_trait]
impl<T> CurveProvider for Client<T>
where
    T: CurveProvider + Send + Sync,
{
    async fn get_tokens_by_format(
        &self,
        request: curve::GetCrvTokenRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_tokens_by_format(request, format, deltas)
            .await
    }

    async fn get_pools_by_format(
        &self,
        request: curve::GetCrvPoolRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_pools_by_format(request, format, deltas)
            .await
    }

    async fn get_prices_by_format(
        &self,
        request: curve::GetCrvPriceRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_prices_by_format(request, format, deltas)
            .await
    }
}

#[async_trait]
impl<T> Erc20Provider for Client<T>
where
    T: Erc20Provider + Send + Sync,
{
    async fn get_erc20_by_format(
        &self,
        request: erc20::GetErc20Request,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_erc20_by_format(request, format, deltas)
            .await
    }

    async fn get_erc20_approval_by_format(
        &self,
        request: erc20::GetErc20ApprovalsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_erc20_approval_by_format(request, format, deltas)
            .await
    }

    async fn get_erc20_transfers_by_format(
        &self,
        request: erc20::GetErc20TransferssRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_erc20_transfers_by_format(request, format, deltas)
            .await
    }
}

#[async_trait]
impl<T> FuelProvider for Client<T>
where
    T: FuelProvider + Send + Sync,
{
    async fn get_fuel_blocks_by_format(
        &self,
        request: fuel::GetFuelBlocksRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_fuel_blocks_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_logs_by_format(
        &self,
        request: fuel::GetFuelLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_fuel_logs_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_logs_decoded_by_format(
        &self,
        request: fuel::GetFuelLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_fuel_logs_decoded_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_txs_by_format(
        &self,
        request: fuel::GetFuelTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_fuel_txs_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_receipts_by_format(
        &self,
        request: fuel::GetFuelReceiptsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_fuel_receipts_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_messages_by_format(
        &self,
        request: fuel::GetFuelMessagesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_fuel_messages_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_unspent_utxos_by_format(
        &self,
        request: fuel::GetUtxoRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_fuel_unspent_utxos_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_spark_markets_by_format(
        &self,
        request: fuel::GetSparkMarketRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_fuel_spark_markets_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_spark_orders_by_format(
        &self,
        request: fuel::GetSparkOrderRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_fuel_spark_orders_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_src20_by_format(
        &self,
        request: fuel::GetSrc20,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_fuel_src20_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_src7_by_format(
        &self,
        request: fuel::GetSrc7,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_fuel_src7_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_mira_v1_pools_by_format(
        &self,
        request: mira::GetMiraPoolsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_fuel_mira_v1_pools_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_mira_v1_liquidity_by_format(
        &self,
        request: mira::GetMiraLiquidityRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_fuel_mira_v1_liquidity_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_mira_v1_swaps_by_format(
        &self,
        request: mira::GetMiraSwapsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_fuel_mira_v1_swaps_by_format(request, format, deltas)
            .await
    }
}

#[async_trait]
impl<T> MoveProvider for Client<T>
where
    T: MoveProvider + Send + Sync,
{
    async fn get_move_logs_by_format(
        &self,
        request: movement::GetMoveLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_move_logs_by_format(request, format, deltas)
            .await
    }

    async fn get_move_logs_decoded_by_format(
        &self,
        request: movement::GetMoveLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_move_logs_decoded_by_format(request, format, deltas)
            .await
    }

    async fn get_move_txs_by_format(
        &self,
        request: movement::GetMoveTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_move_txs_by_format(request, format, deltas)
            .await
    }

    async fn get_move_receipts_by_format(
        &self,
        request: movement::GetMoveReceiptsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_move_receipts_by_format(request, format, deltas)
            .await
    }

    async fn get_move_receipts_decoded_by_format(
        &self,
        request: movement::GetMoveReceiptsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_move_receipts_decoded_by_format(request, format, deltas)
            .await
    }

    async fn get_move_fa_tokens_by_format(
        &self,
        request: movement::GetTokensRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_move_fa_tokens_by_format(request, format, deltas)
            .await
    }

    async fn get_move_interest_v1_pools_by_format(
        &self,
        request: interest::GetPoolsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_move_interest_v1_pools_by_format(request, format, deltas)
            .await
    }

    async fn get_move_interest_v1_liquidity_by_format(
        &self,
        request: interest::GetLiquidityRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_move_interest_v1_liquidity_by_format(request, format, deltas)
            .await
    }

    async fn get_move_interest_v1_swaps_by_format(
        &self,
        request: interest::GetSwapsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_move_interest_v1_swaps_by_format(request, format, deltas)
            .await
    }

    async fn get_move_arche_collaterals_by_format(
        &self,
        request: GetCollateralsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_move_arche_collaterals_by_format(request, format, deltas)
            .await
    }

    async fn get_move_arche_loans_by_format(
        &self,
        request: GetLoansRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_move_arche_loans_by_format(request, format, deltas)
            .await
    }

    async fn get_move_arche_positions_by_format(
        &self,
        request: GetPositionsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_move_arche_positions_by_format(request, format, deltas)
            .await
    }

    async fn get_move_pyth_by_format(
        &self,
        request: pyth::GetPricesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_move_pyth_by_format(request, format, deltas)
            .await
    }

    async fn get_move_balances_by_format(
        &self,
        request: GetBalancesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.check_chain(&request.chains)?;

        self.inner
            .get_move_balances_by_format(request, format, deltas)
            .await
    }
}

#[async_trait]
impl<T> BtcProvider for Client<T>
where
    T: BtcProvider + Send + Sync,
{
    async fn get_btc_blocks_by_format(
        &self,
        request: btc::GetBtcBlocksRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_btc_blocks_by_format(request, format, deltas)
            .await
    }

    async fn get_btc_txs_by_format(
        &self,
        request: btc::GetBtcTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_btc_txs_by_format(request, format, deltas)
            .await
    }
}
