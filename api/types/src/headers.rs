// Copyright © CreditChain Research Team
// SPDX-License-Identifier: Apache-2.0

/// Chain ID of the current chain
pub const X_LIBRA2_CHAIN_ID: &str = "X-Libra2-Chain-Id";
/// Current epoch of the chain
pub const X_LIBRA2_EPOCH: &str = "X-Libra2-Epoch";
/// Current ledger version of the chain
pub const X_LIBRA2_LEDGER_VERSION: &str = "X-libra2-ledger-Version";
/// Oldest non-pruned ledger version of the chain
pub const X_LIBRA2_LEDGER_OLDEST_VERSION: &str = "X-libra2-ledger-Oldest-Version";
/// Current block height of the chain
pub const X_LIBRA2_BLOCK_HEIGHT: &str = "X-Libra2-Block-Height";
/// Oldest non-pruned block height of the chain
pub const X_LIBRA2_OLDEST_BLOCK_HEIGHT: &str = "X-Libra2-Oldest-Block-Height";
/// Current timestamp of the chain
pub const X_LIBRA2_LEDGER_TIMESTAMP: &str = "X-libra2-ledger-TimestampUsec";
/// Cursor used for pagination.
pub const X_LIBRA2_CURSOR: &str = "X-Libra2-Cursor";
/// The cost of the call in terms of gas. Only applicable to calls that result in
/// function execution in the VM, e.g. view functions, txn simulation.
pub const X_LIBRA2_GAS_USED: &str = "X-Libra2-Gas-Used";
/// Provided by the client to identify what client it is.
pub const X_LIBRA2_CLIENT: &str = "x-libra2-client";
