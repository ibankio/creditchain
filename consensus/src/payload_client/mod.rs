// Copyright © CreditChain Research Team
// SPDX-License-Identifier: Apache-2.0

use crate::error::QuorumStoreError;
use creditchain_consensus_types::{common::Payload, payload_pull_params::PayloadPullParameters};
use creditchain_types::validator_txn::ValidatorTransaction;
use creditchain_validator_transaction_pool::TransactionFilter;

pub mod mixed;
pub mod user;
pub mod validator;

#[async_trait::async_trait]
pub trait PayloadClient: Send + Sync {
    async fn pull_payload(
        &self,
        config: PayloadPullParameters,
        validator_txn_filter: TransactionFilter,
    ) -> anyhow::Result<(Vec<ValidatorTransaction>, Payload), QuorumStoreError>;
}
