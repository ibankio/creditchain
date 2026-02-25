// Copyright © CreditChain Research Team
// SPDX-License-Identifier: Apache-2.0

use creditchain_api::context::Context;
use creditchain_config::config::NodeConfig;
use creditchain_mempool::mocks::MockSharedMempool;
use creditchain_storage_interface::mock::MockDbReaderWriter;
use creditchain_types::chain_id::ChainId;
use std::sync::Arc;

// This is necessary for building the API with how the code is structured currently.
pub fn get_fake_context() -> Context {
    let mempool = MockSharedMempool::new_with_runtime();
    Context::new(
        ChainId::test(),
        Arc::new(MockDbReaderWriter),
        mempool.ac_client,
        NodeConfig::default(),
        None, /* table info reader */
    )
}
