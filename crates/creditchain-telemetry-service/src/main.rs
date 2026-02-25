#![forbid(unsafe_code)]

// Copyright © CreditChain Research Team
// SPDX-License-Identifier: Apache-2.0

use creditchain_telemetry_service::CreditChainTelemetryServiceArgs;
use clap::Parser;

#[tokio::main]
async fn main() {
    creditchain_logger::Logger::new().init();
    CreditChainTelemetryServiceArgs::parse().run().await;
}
