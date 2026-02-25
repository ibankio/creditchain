// Copyright © CreditChain Research Team
// SPDX-License-Identifier: Apache-2.0

use creditchain_indexer_grpc_server_framework::ServerArgs;
use creditchain_nft_metadata_crawler::config::NFTMetadataCrawlerConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = <ServerArgs as clap::Parser>::parse();
    args.run::<NFTMetadataCrawlerConfig>().await
}
