import { expect } from "@jest/globals";

export const schema = {
  chain: expect.any(Number),
  block_number: expect.any(String),
  hash: expect.any(String),
  parent_hash: expect.any(String),
  uncles_hash: expect.any(String),
  miner: expect.any(String),
  state_root: expect.any(String),
  transactions_root: expect.any(String),
  receipts_root: expect.any(String),
  gas_used: expect.any(String),
  gas_limit: expect.any(String),
  base_fee_per_gas: expect.any(String),
  extra_data: expect.any(String),
  logs_bloom: expect.any(String),
  timestamp: expect.any(Number),
  difficulty: expect.any(String),
  total_difficulty: expect.any(String),
  size: expect.any(String),
  mix_hash: expect.any(String),
  nonce: expect.any(String),
};
