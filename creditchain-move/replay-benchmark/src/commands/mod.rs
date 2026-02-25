// Copyright (c) A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use creditchain_logger::{Level, Logger};
use creditchain_move_debugger::creditchain_debugger::CreditChainDebugger;
use creditchain_push_metrics::MetricsPusher;
use creditchain_rest_client::{CreditChainBaseUrl, Client};
pub use benchmark::BenchmarkCommand;
use clap::Parser;
pub use diff::DiffCommand;
pub use download::DownloadCommand;
pub use initialize::InitializeCommand;
use url::Url;

mod benchmark;
mod diff;
mod download;
mod initialize;

pub(crate) fn init_logger_and_metrics(log_level: Level) {
    let mut logger = Logger::new();
    logger.level(log_level);
    logger.init();

    let _mp = MetricsPusher::start(vec![]);
}

pub(crate) fn build_debugger(
    rest_endpoint: String,
    api_key: Option<String>,
) -> anyhow::Result<CreditChainDebugger> {
    let builder = Client::builder(CreditChainBaseUrl::Custom(Url::parse(&rest_endpoint)?));
    let client = if let Some(api_key) = api_key {
        builder.api_key(&api_key)?.build()
    } else {
        builder.build()
    };
    CreditChainDebugger::rest_client(client)
}

#[derive(Parser)]
pub struct RestAPI {
    #[clap(
        long,
        help = "Fullnode's REST API query endpoint, e.g., https://api.mainnet.creditchain.io/v1 \
                for mainnet"
    )]
    rest_endpoint: String,

    #[clap(
        long,
        help = "Optional API key to increase HTTP request rate limit quota"
    )]
    api_key: Option<String>,
}
