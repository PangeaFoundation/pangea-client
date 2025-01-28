from dotenv import load_dotenv
from pangea_client import Client
from pangea_client.types import Format
import asyncio
import polars as pl

load_dotenv(override=True)


async def main():
    async with Client(anyone_enabled=True) as client:
        blocks = await client.get_blocks(
            params={
                "chains": ["ETH"],
                "from_block": -10,
                "to_block": "latest",
            },
            format=Format.Arrow,
        )

        df = None
        async for block in blocks:
            if df is None:
                df = pl.read_ipc_stream(block)
            else:
                df = df.extend(pl.read_ipc_stream(block))

            print(df)


if __name__ == "__main__":
    asyncio.run(main())
