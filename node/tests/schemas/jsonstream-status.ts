import { expect } from "@jest/globals";

export const schema = {
  type: "Toolbox",
  chain: expect.any(Number),
  chain_code: expect.any(String),
  chain_name: expect.any(String),
  entity: expect.any(String),
  latest_block_height: expect.any(Number),
  service: expect.any(String),
  status: expect.any(String),
  timestamp: expect.any(Number),
};
