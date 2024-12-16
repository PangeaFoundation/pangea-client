## Pangea Client

`pangea-client` is a set of libraries for different languages to interface with [Pangea API](https://docs.pangea.foundation/).

<br>

## Getting Started

See the README in each language directory for specific details.

This project is a monorepo, providing implementations for multiple languages:

- `rust` &nbsp;&nbsp;&nbsp; - [README.md](./rust/README.md)
- `python` - [README.md](./python/README.md)
- `node` &nbsp;&nbsp;&nbsp; - [README.md](./node/README.md)

<br>

Access to the API via `pangea-client` requires credentials, please [apply for access](https://pangea.foundation/get-access) first.

Once credentials are issued, they will need to be set in the environment variables.

The easiest way to use these credentials is to create a `.env` file in the project root folder and populate it like so:

```sh
PANGEA_USERNAME=xxxxx
PANGEA_PASSWORD=xxxxx
```

<br>

## Installation

You can use the popular package registries for each language to install the package's official distribution:

- `rust` <br>

  ```sh
  cargo add pangea-client
  ```

  <br>

- `python` <br>

  ```sh
  pip install pangea-client
  ```

  <br>

- `node` <br>

  ```sh
  npm install pangea-client
  ```

<br>

## Examples

`rust`

```rust
use futures::StreamExt;
use pangea_client::{
    provider::ChainProvider, core::types::ChainId, query::Bound,
    requests::blocks::GetBlocksRequest, ClientBuilder, Format, WsProvider,
};
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv_override().ok();

    let client = ClientBuilder::default()
        .build::<WsProvider>()
        .await
        .unwrap();

    let request = GetBlocksRequest {
        chains: HashSet::from([ChainId::ETH]),
        from_block: Bound::FromLatest(10),
        to_block: Bound::Latest,
        ..Default::default()
    };

    let stream = client
        .get_blocks_by_format(request, Format::JsonStream, false)
        .await
        .unwrap();
    futures::pin_mut!(stream);

    while let Some(Ok(data)) = stream.next().await {
        let data = String::from_utf8(data).unwrap();
        println!("data: {data}");
    }

    Ok(())
}
```

<br>

`python`

```python
from dotenv import load_dotenv
from pangea_client import Client
from pangea_client.types import Format
import asyncio
import json

load_dotenv(override=True)


async def main():
    async with Client() as client:
        handle = await client.get_blocks(
            params={
                "chains": ["ETH"],
                "from_block": -10,
                "to_block": "latest",
            },
            format=Format.JsonStream,
        )

        async for chunk in handle:
            lines = chunk.strip().split("\n")
            for line in lines:
                if line:
                    print(json.loads(line))


if __name__ == "__main__":
    asyncio.run(main())
```

<br>

`node`

```javascript
import { Client, RequestFormats } from "pangea-client";

require("dotenv").config({ override: true });

export async function main(): Promise<void> {
  const client = await Client.build();

  const handle = await client.get_blocks(
    {
      chains: ["ETH"],
      from_block: -10,
      to_block: "latest",
    },
    RequestFormats.JSON_STREAM
  );

  try {
    for await (const chunk of handle) {
      chunk
        .toString()
        .split("\n")
        .filter(Boolean)
        .forEach((line: any) => {
          console.log(JSON.parse(line));
        });
    }
  } finally {
    client.disconnect();
  }
}
```
