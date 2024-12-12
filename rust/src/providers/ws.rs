use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use futures::{select_biased, stream::Fuse, FutureExt, SinkExt, StreamExt, TryStreamExt};
use http::header;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_stream::wrappers::{ReceiverStream, UnboundedReceiverStream};
use tokio_tungstenite::{connect_async_with_config, MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, warn};
use tungstenite::{client::IntoClientRequest, protocol::WebSocketConfig, Message};
use uuid::Uuid;

use crate::{
    core::{
        error::{Error, ResponseError, Result},
        types::format::Format,
    },
    provider::{
        BtcProvider, ChainProvider, CurveProvider, Erc20Provider, FuelProvider, Provider,
        StreamResponse, UniswapV2Provider, UniswapV3Provider,
    },
    requests::{blocks, btc, curve, erc20, fuel, logs, transfers, txs, uniswap_v2, uniswap_v3},
    ChainId,
};

const WS_PATH: &str = "v1/websocket";

type WsResult = Result<Vec<u8>>;
type OperationMsg = (Request, mpsc::Sender<WsResult>);

#[derive(Clone, Debug)]
pub struct WsProvider {
    operations: mpsc::UnboundedSender<OperationMsg>,
}

impl WsProvider {
    pub async fn request(
        &self,
        operation: Operation,
        params: impl Serialize,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        let (sink, stream) = mpsc::channel(5);
        let id = Uuid::new_v4();

        let params = serde_json::to_value(params).map_err(Error::from)?;
        let params = if matches!(params, serde_json::Value::Null) {
            HashMap::new()
        } else {
            serde_json::from_value(params).map_err(Error::from)?
        };
        let request = Request {
            id,
            operation,
            params,
            format,
            deltas,
        };
        self.operations
            .send((request, sink))
            .map_err(|_| Error::BackendShutDown)?;

        let stream = ReceiverStream::new(stream)
            .map_err(Error::from)
            .filter_map(|data| async {
                match data {
                    Ok(data) if !data.is_empty() => Some(Ok(data)),
                    Ok(_) => None,
                    Err(e) => Some(Err(e)),
                }
            })
            .boxed();

        Ok(stream)
    }

    /// Returns true if the WS connection is active, false otherwise
    pub fn ready(&self) -> bool {
        !self.operations.is_closed()
    }
}

#[async_trait]
impl Provider for WsProvider {
    async fn try_new(
        endpoint: String,
        is_secure: bool,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<Self> {
        let mut req = format!(
            "{}://{endpoint}/{WS_PATH}",
            if is_secure { "wss" } else { "ws" },
        )
        .into_client_request()?;

        if let (Some(username), Some(password)) = (username, password) {
            let auth = format!("{username}:{password}");
            let encoded = BASE64.encode(auth);

            req.headers_mut().append(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Basic {encoded}"))
                    .expect("Only non-ascii chars result in an error"),
            );
        }

        let (sink, stream) = mpsc::unbounded_channel();
        let bw = BackgroundWorker::new(req, stream).await?;
        tokio::spawn(bw.main_loop());

        Ok(Self { operations: sink })
    }

    async fn get_status_by_format(&self, format: Format) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetStatus, (), format, false).await
    }
}

#[async_trait]
impl ChainProvider for WsProvider {
    async fn get_blocks_by_format(
        &self,
        request: blocks::GetBlocksRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetBlocks, request, format, deltas)
            .await
    }

    async fn get_logs_by_format(
        &self,
        request: logs::GetLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetLogs, request, format, deltas)
            .await
    }

    async fn get_txs_by_format(
        &self,
        request: txs::GetTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetTxs, request, format, deltas)
            .await
    }

    async fn get_transfers_by_format(
        &self,
        request: transfers::GetTransfersRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetTransfers, request, format, deltas)
            .await
    }
}

#[async_trait]
impl UniswapV2Provider for WsProvider {
    async fn get_pairs_by_format(
        &self,
        request: uniswap_v2::GetPairsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetUniswapV2Pairs, request, format, deltas)
            .await
    }

    async fn get_prices_by_format(
        &self,
        request: uniswap_v2::GetPricesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetUniswapV2Prices, request, format, deltas)
            .await
    }
}

#[async_trait]
impl UniswapV3Provider for WsProvider {
    async fn get_fees_by_format(
        &self,
        request: uniswap_v3::GetFeesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetUniswapV3Fees, request, format, deltas)
            .await
    }

    async fn get_pools_by_format(
        &self,
        request: uniswap_v3::GetPoolsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetUniswapV3Pools, request, format, deltas)
            .await
    }

    async fn get_positions_by_format(
        &self,
        request: uniswap_v3::GetPositionsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetUniswapV3Positions, request, format, deltas)
            .await
    }

    async fn get_prices_by_format(
        &self,
        request: uniswap_v3::GetPricesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetUniswapV3Prices, request, format, deltas)
            .await
    }
}

#[async_trait]
impl CurveProvider for WsProvider {
    async fn get_tokens_by_format(
        &self,
        request: curve::GetCrvTokenRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetCurveTokens, request, format, deltas)
            .await
    }

    async fn get_pools_by_format(
        &self,
        request: curve::GetCrvPoolRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetCurvePools, request, format, deltas)
            .await
    }

    async fn get_prices_by_format(
        &self,
        request: curve::GetCrvPriceRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetCurvePrices, request, format, deltas)
            .await
    }
}

#[async_trait]
impl Erc20Provider for WsProvider {
    async fn get_erc20_by_format(
        &self,
        request: erc20::GetErc20Request,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetErc20, request, format, deltas)
            .await
    }

    async fn get_erc20_approval_by_format(
        &self,
        request: erc20::GetErc20ApprovalsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetErc20Approvals, request, format, deltas)
            .await
    }

    async fn get_erc20_transfers_by_format(
        &self,
        request: erc20::GetErc20TransferssRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetErc20Transfers, request, format, deltas)
            .await
    }
}

#[async_trait]
impl FuelProvider for WsProvider {
    async fn get_fuel_blocks_by_format(
        &self,
        request: fuel::GetFuelBlocksRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetBlocks, request, format, deltas)
            .await
    }

    async fn get_fuel_logs_by_format(
        &self,
        request: fuel::GetFuelLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetLogs, request, format, deltas)
            .await
    }

    async fn get_fuel_logs_decoded_by_format(
        &self,
        request: fuel::GetFuelLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetFuelLogsDecoded, request, format, deltas)
            .await
    }

    async fn get_fuel_txs_by_format(
        &self,
        request: fuel::GetFuelTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetTxs, request, format, deltas)
            .await
    }

    async fn get_fuel_receipts_by_format(
        &self,
        request: fuel::GetFuelReceiptsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetReceipts, request, format, deltas)
            .await
    }

    async fn get_fuel_messages_by_format(
        &self,
        request: fuel::GetFuelMessagesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetMessages, request, format, deltas)
            .await
    }

    async fn get_fuel_unspent_utxos_by_format(
        &self,
        request: fuel::GetUtxoRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetUnspentUtxos, request, format, deltas)
            .await
    }

    async fn get_fuel_spark_markets_by_format(
        &self,
        request: fuel::GetSparkMarketRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetSparkMarket, request, format, deltas)
            .await
    }

    async fn get_fuel_spark_orders_by_format(
        &self,
        request: fuel::GetSparkOrderRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetSparkOrder, request, format, deltas)
            .await
    }

    async fn get_fuel_src20_by_format(
        &self,
        request: fuel::GetSrc20,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetSrc20, request, format, deltas)
            .await
    }

    async fn get_fuel_src7_by_format(
        &self,
        request: fuel::GetSrc7,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetSrc7, request, format, deltas)
            .await
    }
}

#[async_trait]
impl BtcProvider for WsProvider {
    async fn get_btc_blocks_by_format(
        &self,
        mut request: btc::GetBtcBlocksRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        request.chains = HashSet::from_iter(vec![ChainId::BTC]);
        self.request(Operation::GetBlocks, request, format, deltas)
            .await
    }

    async fn get_btc_txs_by_format(
        &self,
        mut request: btc::GetBtcTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        request.chains = HashSet::from_iter(vec![ChainId::BTC]);
        self.request(Operation::GetTxs, request, format, deltas)
            .await
    }
}

struct BackgroundWorker {
    ws: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    operations: Fuse<UnboundedReceiverStream<OperationMsg>>,
    subscriptions: HashMap<Uuid, mpsc::Sender<Result<Vec<u8>>>>,
}

impl BackgroundWorker {
    pub async fn new(
        ws_server: http::Request<()>,
        operations: mpsc::UnboundedReceiver<OperationMsg>,
    ) -> Result<Self> {
        let config = WebSocketConfig {
            max_frame_size: None,
            max_message_size: None,
            ..Default::default()
        };
        let (ws, _) = connect_async_with_config(ws_server, Some(config), false).await?;
        let operations = UnboundedReceiverStream::new(operations).fuse();

        Ok(Self {
            ws,
            operations,
            subscriptions: HashMap::default(),
        })
    }

    pub async fn main_loop(mut self) {
        let Err(err) = self.try_run().await else {
            return;
        };

        error!("Websocket connection failed: {err}");

        let err = err.to_string();
        for sub in self.subscriptions.values_mut() {
            let _ = sub.send(Err(Error::ErrorMsg(err.clone()))).await;
        }
    }

    async fn try_run(&mut self) -> Result<()> {
        let mut ping_interval = tokio::time::interval(Duration::from_secs(5));
        let mut latest_msg_stamp = Instant::now();

        loop {
            select_biased! {
                _ = ping_interval.tick().fuse() => {
                    if latest_msg_stamp.elapsed().as_secs_f64() > 9.0 {
                        return Err(Error::PingTimeout);
                    }
                    self.ws.send(Message::Ping(Vec::new())).await?
                }
                operation = self.operations.next() => {
                    match operation {
                        Some(operation) => self.operate(operation).await?,
                        None => {
                            self.ws.close(None).await?;
                            return Ok(())
                        },
                    }
                }
                resp = self.ws.try_next() => {
                    match resp {
                        Ok(Some(message)) => {
                            latest_msg_stamp = Instant::now();
                            self.handle(message).await?;
                        }
                        Ok(None) => return Err(Error::UnexpectedClose),
                        Err(e) => return Err(Error::Tungstenite(e)),
                    }
                }
            }
        }
    }

    async fn operate(&mut self, operation: OperationMsg) -> Result<()> {
        let (request, sink) = operation;
        let payload = serde_json::to_vec(&request)?;

        if self.subscriptions.insert(request.id, sink).is_some() {
            warn!(
                "Replacing already-registered subscription with id {:?}",
                request.id
            );
        }

        self.ws.send(Message::Binary(payload)).await?;

        Ok(())
    }

    async fn handle(&mut self, resp: Message) -> Result<()> {
        match resp {
            Message::Text(_) => Err(Error::UnexpectedMessage),
            Message::Frame(_) => Ok(()), // Server is allowed to send Raw frames
            Message::Ping(inner) => self.handle_ping(inner).await,
            Message::Pong(_) => Ok(()), // Server is allowed to send unsolicited pongs.
            Message::Close(_) => Err(Error::UnexpectedClose),
            Message::Binary(buf) => self.handle_binary(buf).await,
        }
    }

    async fn handle_ping(&mut self, inner: Vec<u8>) -> Result<()> {
        self.ws.send(Message::Pong(inner)).await?;
        Ok(())
    }

    async fn handle_binary(&mut self, data: Vec<u8>) -> Result<()> {
        let (header, data) = Header::try_from_data(data)?;
        let id = header.id;

        let msg = match header.kind {
            Kind::Start => {
                return Ok(());
            }
            Kind::Continue => Ok(data),
            Kind::ContinueWithError => match data.first() {
                Some(b'{') => match serde_json::from_slice::<ResponseError>(&data) {
                    Ok(err) => Err(Error::ErrorResponse(err)),
                    Err(_) => Err(Error::UnexpectedMessageFormat),
                },
                _ => match String::from_utf8(data) {
                    Ok(s) => Err(Error::ErrorMsg(s)),
                    Err(_) => Err(Error::UnexpectedMessageFormat),
                },
            },
            Kind::End => {
                debug!("Subscription with id {:?} ended", id);
                self.subscriptions.remove(&header.id.0);
                return Ok(());
            }
            Kind::Error => match String::from_utf8(data) {
                Ok(s) => Err(Error::ErrorMsg(s)),
                Err(_) => Err(Error::UnexpectedMessageFormat),
            },
            _ => Err(Error::UnexpectedMessageFormat),
        };

        if let std::collections::hash_map::Entry::Occupied(mut occupied) =
            self.subscriptions.entry(id.0)
        {
            if let Err(err) = occupied.get_mut().send(msg).await {
                return Err(Error::Custom(
                    format!("failed to send message: {err:?}").into(),
                ));
            }
        }

        Ok(())
    }
}

#[derive(Clone, serde::Serialize)]
struct Request {
    id: Uuid,
    operation: Operation,
    #[serde(flatten)]
    params: HashMap<String, serde_json::Value>,
    #[serde(default)]
    format: Format,
    #[serde(default)]
    deltas: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Operation {
    GetStatus,
    GetBlocks,
    GetLogs,
    GetTxs,
    GetReceipts,
    #[serde(rename = "getDecodedLogs")]
    GetFuelLogsDecoded,
    GetMessages,
    GetUnspentUtxos,
    GetUniswapV2Pairs,
    GetUniswapV2Prices,
    GetUniswapV3Fees,
    GetUniswapV3Pools,
    GetUniswapV3Positions,
    GetUniswapV3Prices,
    GetCurveTokens,
    GetCurvePools,
    GetCurvePrices,
    GetTransfers,
    GetErc20,
    GetErc20Approvals,
    GetErc20Transfers,
    GetSparkMarket,
    GetSparkOrder,
    GetSrc20,
    GetSrc7,
}

#[derive(Debug, Clone, Deserialize)]
struct Header {
    pub kind: Kind,
    pub id: MsgId,
    #[serde(rename = "counter")]
    pub _counter: u32,
    #[serde(rename = "epoch")]
    pub _epoch: Option<u64>,
}

impl Header {
    fn try_from_data(mut data: Vec<u8>) -> Result<(Self, Vec<u8>)> {
        // seperate by new line
        let mut split = data.splitn_mut(2, |b| *b == b'\n');
        let header = split.next().ok_or_else(|| Error::UnexpectedMessageFormat)?;
        let data = split.next().ok_or_else(|| Error::UnexpectedMessageFormat)?;
        let header = serde_json::from_slice::<Header>(header)?;
        Ok((header, data.to_vec()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Kind {
    Start,
    Continue,
    ContinueWithError,
    End,
    Error,
    Subscription,
}

/// An id describing a subscription or a response
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MsgId(pub Uuid);
