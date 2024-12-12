import crypto from "crypto";
import { Buffer } from "buffer";
import { WebSocket } from "ws";

export interface ClientOptions {
  endpoint?: string;
  username?: string;
  password?: string;
  isSecure?: boolean;
  timeout?: number;
}

function applyDefaults(options: ClientOptions): ClientOptions {
  return {
    username: options.username,
    password: options.password,
    endpoint: options.endpoint || DEFAULT_ENDPOINT,
    isSecure: options.isSecure === undefined ? true : options.isSecure,
    timeout: options.timeout || 5000,
  };
}

const UUID_NIL = "00000000-0000-0000-0000-000000000000";
const DEFAULT_ENDPOINT = "app.pangea.foundation";
const PING_INTERVAL = 5_000;
const WAIT_INTERVAL = 0;

const methodsToEndpointsWithoutParams = [["get_status", "getStatus"]] as const;

const methodsToEndpointsWithParams = [
  ["get_blocks", "getBlocks"],
  ["get_logs", "getLogs"],
  ["get_logs_decoded", "getDecodedLogs"],
  ["get_transactions", "getTxs"],
  ["get_receipts", "getReceipts"],
  ["get_contracts", "getContracts"],
  ["get_uniswap_v2_pairs", "getUniswapV2Pairs"],
  ["get_uniswap_v2_prices", "getUniswapV2Prices"],
  ["get_uniswap_v3_pools", "getUniswapV3Pools"],
  ["get_uniswap_v3_fees", "getUniswapV3Fees"],
  ["get_uniswap_v3_positions", "getUniswapV3Positions"],
  ["get_uniswap_v3_prices", "getUniswapV3Prices"],
  ["get_curve_tokens", "getCurveTokens"],
  ["get_curve_pools", "getCurvePools"],
  ["get_curve_prices", "getCurvePrices"],
  ["get_transfers", "getTransfers"],
  ["get_erc20_tokens", "getErc20"],
  ["get_erc20_approvals", "getErc20Approvals"],
  ["get_erc20_transfers", "getErc20Transfers"],
  ["get_fuel_spark_markets", "getSparkMarket"],
  ["get_fuel_spark_orders", "getSparkOrder"],
  ["get_fuel_unspent_utxos", "getUnspentUtxos"],
  ["get_fuel_src20_metadata", "getSrc20"],
  ["get_fuel_src7_metadata", "getSrc7"],
] as const;

type MethodWithParams = (typeof methodsToEndpointsWithParams)[number][0];
type MethodWithoutParams = (typeof methodsToEndpointsWithoutParams)[number][0];

type ClientMethodsWithoutParams = {
  [K in MethodWithoutParams]: (format: RequestFormats) => Promise<any>;
};
type ClientMethodsWithParams = {
  [K in MethodWithParams]: (
    params: {},
    format?: RequestFormats,
    deltas?: boolean,
  ) => Promise<any>;
};

type Header = {
  id: string;
  kind: string;
  cursor?: string;
};

export enum RequestFormats {
  ARROW = "arrow",
  ARROW_STREAM = "arrow_stream",
  JSON = "json",
  JSON_STREAM = "json_stream", // default
}

export type ClientWithEndpoints = Client & ClientMethodsWithoutParams & ClientMethodsWithParams;

export class Client {
  public readonly endpoint: string;
  private _connection: WebSocket;
  private _timeout: number;
  private _data_queues: Map<string, { header: Header; body: Buffer }[]>;

  public get connection(): WebSocket {
    return this._connection;
  }

  static async build(options?: ClientOptions): Promise<ClientWithEndpoints> {
    options = applyDefaults(options ?? {});

    const client = new Client(options);

    // wait for connection to establish
    while (client.connection.readyState === WebSocket.CONNECTING) {
      await new Promise((resolve) => setTimeout(resolve, WAIT_INTERVAL));
    }

    if (client.connection.readyState !== WebSocket.OPEN) {
      throw new Error("Failed to create client");
    }

    // bind the methods dynamically with their specific endpoints

    for (const [method, endpoint] of methodsToEndpointsWithoutParams) {
      (client as any)[method] = async (format = RequestFormats.JSON_STREAM) =>
        client.send_request(endpoint, {}, format, false);
    }

    for (const [method, endpoint] of methodsToEndpointsWithParams) {
      (client as any)[method] = async (
        params: {},
        format = RequestFormats.JSON_STREAM,
        deltas = false,
      ) => client.send_request(endpoint, params, format, deltas);
    }

    return client as unknown as ClientWithEndpoints;
  }

  private constructor(options: ClientOptions) {
    const username = options.username || process.env.PANGEA_USERNAME;
    if (!username) {
      throw new Error("Missing PANGEA_USERNAME in environment variables");
    }

    const password = options.password || process.env.PANGEA_PASSWORD;
    if (!password) {
      throw new Error("Missing PANGEA_PASSWORD in environment variables");
    }

    const endpoint = options.endpoint || process.env.PANGEA_URL;
    const protocol = `ws${options.isSecure ? "s" : ""}`;
    this.endpoint = `${protocol}://${username}:${password}@${endpoint}/v1/websocket`;

    // console.log("");
    // console.log(`Username: \x1b[1m${username}\x1b[0m`);
    // console.log(`URL: \x1b[1m${endpoint}\x1b[0m`);
    // console.log("");

    this._data_queues = new Map();

    this._timeout = options.timeout!;
    this._connection = this.createConnection();
  }

  private createConnection() {
    const connection = new WebSocket(this.endpoint, {
      maxPayload: 1024 * 1024 * 1024, // 1GB
    });

    const timeoutId = setTimeout(() => {
      console.error("WebSocket connection timed out");

      connection.removeEventListener("error", onError);
      connection.removeEventListener("open", onOpen);
      connection.terminate();
    }, this._timeout);

    const onError = (error: any) => {
      console.error("WebSocket connection encountered an error:", error);

      clearTimeout(timeoutId);

      connection.removeEventListener("error", onError);
      connection.removeEventListener("error", onOpen);
      connection.terminate();
    };

    let alive = false; // track connection liveness via pinging

    const onOpen = () => {
      const pingingLoop = setInterval(() => {
        if (connection.readyState !== WebSocket.OPEN) {
          clearInterval(pingingLoop);

          return;
        }

        if (alive) {
          alive = false;
          connection.ping();

          return;
        }

        // no pong (nor message) received in time, closing connection
        console.error("WebSocket connection lost...");

        clearInterval(pingingLoop);
        connection.close();
      }, PING_INTERVAL);

      connection.on("pong", () => {
        alive = true;
      });

      connection.on("close", () => {
        clearInterval(pingingLoop);
      });

      alive = true;
      clearTimeout(timeoutId);
      connection.removeEventListener("open", onOpen);
    };

    // store responses from server into their dedicated message queues
    const onMessage = (raw_data: Buffer) => {
      alive = true; // keep alive

      const newlineIndex = raw_data.indexOf("\n");
      if (newlineIndex === -1) return;

      const headerJSON = raw_data.subarray(0, newlineIndex).toString();
      const body = raw_data.subarray(newlineIndex + 1); // preserving the body as bytes

      const header = JSON.parse(headerJSON);

      if (header.id === UUID_NIL && header.kind === "Error") {
        throw new Error(body.toString());
      }

      this._data_queues.get(header.id)!.push({ header, body });
    };

    connection.addEventListener("open", onOpen);
    connection.addEventListener("error", onError);
    connection.on("message", onMessage);

    return connection;
  }

  async connect() {
    this.disconnect();

    while (this._connection.readyState !== WebSocket.CLOSED) {
      await new Promise((resolve) => setTimeout(resolve, WAIT_INTERVAL));
    }

    this._connection = this.createConnection();
  }

  async disconnect() {
    this._connection.close();
    this._data_queues.clear();
  }

  async send_request(operation: string, params: {}, format: RequestFormats, deltas: boolean) {
    if (this._connection.readyState !== WebSocket.OPEN) {
      throw new Error("WebSocket connection expected to be 'OPEN' when sending a request");
    }

    const id = crypto.randomUUID();

    const request = {
      id,
      cursor: null as string | null | undefined,
      operation,
      ...params,
      format,
      deltas,
    };

    this._data_queues.set(id, []);
    this._connection.send(JSON.stringify(request));

    return this.handle_request(id);
  }

  async *handle_request(id: string) {
    const queue = this._data_queues.get(id);
    if (!queue) return;

    while (this._connection.readyState === WebSocket.OPEN) {
      if (queue.length === 0) {
        await new Promise((resolve) => setTimeout(resolve, WAIT_INTERVAL));

        continue;
      }

      const item = queue.shift();
      if (!item) break;

      const { header, body } = item;

      if (header.kind && header.kind.startsWith("Continue")) {
        yield body;
      } else if (header.kind === "Start") {
        continue;
      } else {
        this._data_queues.delete(id);

        if (header.kind === "End") {
          break;
        }

        this.disconnect();

        if (header.kind === "Error") {
          throw new Error(body.toString());
        }

        throw new Error(`Unexpected kind of response from server: ${header.kind}`);
      }
    }
  }
}
