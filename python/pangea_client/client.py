import asyncio
import base64
import json
import os
import uuid
import websockets

from .types import Format
from enum import Enum
from typing import Optional

UUID_NIL = "00000000-0000-0000-0000-000000000000"
DEFAULT_ENDPOINT = "app.pangea.foundation"

methods_to_endpoints_without_params = [
    ("get_status", "getStatus"),
]

methods_to_endpoints_with_params = [
    ("get_blocks", "getBlocks"),
    ("get_logs", "getLogs"),
    ("get_logs_decoded", "getDecodedLogs"),
    ("get_transactions", "getTxs"),
    ("get_receipts", "getReceipts"),
    ("get_contracts", "getContracts"),
    ("get_uniswap_v2_pairs", "getUniswapV2Pairs"),
    ("get_uniswap_v2_prices", "getUniswapV2Prices"),
    ("get_uniswap_v3_pools", "getUniswapV3Pools"),
    ("get_uniswap_v3_fees", "getUniswapV3Fees"),
    ("get_uniswap_v3_positions", "getUniswapV3Positions"),
    ("get_uniswap_v3_prices", "getUniswapV3Prices"),
    ("get_curve_tokens", "getCurveTokens"),
    ("get_curve_pools", "getCurvePools"),
    ("get_curve_prices", "getCurvePrices"),
    ("get_transfers", "getTransfers"),
    ("get_erc20_tokens", "getErc20"),
    ("get_erc20_approvals", "getErc20Approvals"),
    ("get_erc20_transfers", "getErc20Transfers"),
    ("get_fuel_spark_markets", "getSparkMarket"),
    ("get_fuel_spark_orders", "getSparkOrder"),
    ("get_fuel_unspent_utxos", "getUnspentUtxos"),
    ("get_fuel_src20_metadata", "getSrc20"),
    ("get_fuel_src7_metadata", "getSrc7"),
]


class Kind(Enum):
    START = "Start"
    CONTINUE = "Continue"
    CONTINUE_WITH_ERROR = "ContinueWithError"
    END = "End"
    ERROR = "Error"


class Client:
    def __init__(
        self,
        endpoint=None,
        username=None,
        password=None,
        is_secure=True,
    ):
        self.username = username or os.getenv("PANGEA_USERNAME")
        if not self.username:
            raise ValueError("Missing PANGEA_USERNAME in environment variables")

        self.password = password or os.getenv("PANGEA_PASSWORD")
        if not self.password:
            raise ValueError("Missing PANGEA_PASSWORD in environment variables")

        self.is_secure = is_secure
        self.endpoint = f"ws{'s' if is_secure else ''}://{endpoint or os.getenv('PANGEA_URL', DEFAULT_ENDPOINT)}/v1/websocket"
        self.connection: Optional[websockets.WebSocketClientProtocol] = None
        self.request_handlers = {}
        self.receive_task = None
        self._shutdown_signal = asyncio.Event()

    async def __aenter__(self):
        await self.connect()
        self.receive_task = asyncio.create_task(self.receive_loop())

        # bind the method dynamically with the endpoints

        def create_method_without_params(endpoint):
            async def method_instance(self, format=Format.JsonStream):
                return await self.send_request(operation=endpoint, format=format, deltas=False)

            return method_instance

        for method_name, endpoint in methods_to_endpoints_without_params:
            setattr(
                self,
                method_name,
                create_method_without_params(endpoint).__get__(self, type(self)),
            )

        def create_method_with_params(endpoint):
            async def method_instance(
                self, params: dict, deltas=False, format=Format.JsonStream
            ):
                return await self.send_request(
                    operation=endpoint, params=params, format=format, deltas=deltas
                )

            return method_instance

        for method_name, endpoint in methods_to_endpoints_with_params:
            setattr(
                self, method_name, create_method_with_params(endpoint).__get__(self, type(self))
            )

        return self

    async def __aexit__(self, exc_type, exc_value, traceback):
        await self.disconnect()

    async def connect(self):
        if self.connection:
            await self.disconnect()

        headers = {}
        if self.username and self.password:
            credentials = base64.b64encode(
                f"{self.username}:{self.password}".encode("utf-8")
            ).decode("utf-8")
            headers["Authorization"] = f"Basic {credentials}"

        self.connection = await websockets.connect(
            self.endpoint,
            max_size=None,
            ping_interval=5,
            ping_timeout=9,
            extra_headers=headers,
        )
        self._shutdown_signal.clear()

    async def disconnect(self):
        self._shutdown_signal.set()

        # close pending request handlers
        for queue in self.request_handlers.values():
            try:
                while not queue.empty():
                    queue.get_nowait()

                queue.put_nowait(None)  # signal completion

            except asyncio.QueueEmpty:
                pass

            except asyncio.QueueFull:
                print(f"Error while signlaing completion: queue is full")
                pass  # skip putting the completion signal ¯\_(ツ)_/¯

        # cancel receive loop task
        if self.receive_task:
            self.receive_task.cancel()
            try:
                await self.receive_task
            except (asyncio.CancelledError, websockets.exceptions.ConnectionClosedError):
                pass

        # close the WebSocket connection
        if self.connection:
            try:
                await self.connection.close()
            except (websockets.exceptions.WebSocketException, ConnectionError) as e:
                print(f"Error while closing connection: {e}")
                pass

            self.connection = None

    async def receive_loop(self):
        while True:
            try:
                response = await asyncio.wait_for(self.connection.recv(), timeout=5.0)
                response_parts = response.split(b"\n", 1)
                if not response_parts:
                    raise Exception("Unexpected response from server")

                header = json.loads(response_parts[0])
                header_id = header["id"]
                data = response_parts[1] if len(response_parts) > 1 else None

                if header_id == UUID_NIL and header["kind"] == Kind.ERROR.value:
                    raise Exception("Invalid request: " + data.decode("utf-8"))

                if header_id in self.request_handlers:
                    await self.request_handlers[header_id].put(response)

            except asyncio.TimeoutError:
                continue

            except websockets.exceptions.ConnectionClosedError as e:
                print(f"Connection closed: {e}")
                break

            except Exception as e:
                print(f"Error in receive loop: {e}")
                break

        await self.disconnect()

    async def send_request(
        self,
        operation=None,
        params={},
        format=Format.JsonStream,
        deltas=False,
    ):
        if not self.connection or not self.connection.open:
            raise Exception("WebSocket connection expected to be 'OPEN' when sending a request")

        if operation is None:
            raise Exception("Operation is required for a new request")

        if type(format) is not Format:
            raise Exception("Invalid format type")

        id = str(uuid.uuid4())
        request = {
            "id": id,
            "operation": operation,
            "deltas": deltas,
            "format": format.value,
            **params,
        }

        self.request_handlers[id] = asyncio.Queue()

        await self.connection.send(json.dumps(request))

        return self.handle_request(id, format)

    async def handle_request(self, id, format):
        while self.connection.open:
            # get the queue
            queue = self.request_handlers.get(id)
            if queue is None:
                break

            # get response from queue
            response = await queue.get()
            if response is None:
                break

            # process response
            response_parts = response.split(b"\n", 1)
            if len(response_parts) == 0:
                raise Exception("Unexpected response from server")

            header = json.loads(response_parts[0])
            data = response_parts[1] if len(response_parts) > 1 else b""

            header_id = header["id"]
            kind = header["kind"]

            if header_id != id:
                continue

            if kind == Kind.START.value:
                continue

            elif kind in (Kind.CONTINUE.value, Kind.CONTINUE_WITH_ERROR.value):
                if format in (Format.Arrow, Format.ArrowStream):
                    yield data
                else:
                    yield data.decode("utf-8")

            elif kind == Kind.ERROR.value:
                raise Exception(data.decode("utf-8"))

            elif kind == Kind.END.value:
                break

            else:
                raise Exception(f"Unexpected response kind: {kind}")

        # cleanup handlers
        self.request_handlers.pop(id, None)
