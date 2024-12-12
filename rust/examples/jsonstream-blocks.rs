use futures::StreamExt;
use pangea_client::{
    core::types::ChainId, provider::ChainProvider, requests::blocks::GetBlocksRequest,
    ClientBuilder, Format, WsProvider,
};
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv_override().ok();

    let client = ClientBuilder::default()
        .build::<WsProvider>()
        .await
        .unwrap();

    // historic request -- 1000 blocks
    // let request = GetBlocksRequest {
    //    from_block: Bound::Exact(17_000_000),
    //    to_block: Bound::Exact(17_001_000),
    //    ..Default::default()
    // }

    // historic request -- last 100 blocks
    // let request = GetBlocksRequest {
    //    from_block: Bound::FromLatest(100),
    //    to_block: Bound::Latest,
    //    ..Default::default()
    // };

    // stream realtime
    // let request = GetBlocksRequest {
    //     from_block: Bound::Latest,
    //     to_block: Bound::Subscribe,
    //     chains: HashSet::from([ChainId::ETH]),
    //     ..Default::default()
    // };

    // get lastest block available
    let request = GetBlocksRequest {
        chains: HashSet::from([ChainId::ETH]), // chains can be specified here
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
