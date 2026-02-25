// Copyright © CreditChain Research Team
// SPDX-License-Identifier: Apache-2.0

/// Chain ID of the current chain
pub const X_CREDITCHAIN_CHAIN_ID: &str = "X-CreditChain-Chain-Id";
/// Current epoch of the chain
pub const X_CREDITCHAIN_EPOCH: &str = "X-CreditChain-Epoch";
/// Current ledger version of the chain
pub const X_CREDITCHAIN_LEDGER_VERSION: &str = "X-creditchain-ledger-Version";
/// Oldest non-pruned ledger version of the chain
pub const X_CREDITCHAIN_LEDGER_OLDEST_VERSION: &str = "X-creditchain-ledger-Oldest-Version";
/// Current block height of the chain
pub const X_CREDITCHAIN_BLOCK_HEIGHT: &str = "X-CreditChain-Block-Height";
/// Oldest non-pruned block height of the chain
pub const X_CREDITCHAIN_OLDEST_BLOCK_HEIGHT: &str = "X-CreditChain-Oldest-Block-Height";
/// Current timestamp of the chain
pub const X_CREDITCHAIN_LEDGER_TIMESTAMP: &str = "X-creditchain-ledger-TimestampUsec";
/// Cursor used for pagination.
pub const X_CREDITCHAIN_CURSOR: &str = "X-CreditChain-Cursor";
/// The cost of the call in terms of gas. Only applicable to calls that result in
/// function execution in the VM, e.g. view functions, txn simulation.
pub const X_CREDITCHAIN_GAS_USED: &str = "X-CreditChain-Gas-Used";
/// Provided by the client to identify what client it is.
pub const X_CREDITCHAIN_CLIENT: &str = "x-creditchain-client";
