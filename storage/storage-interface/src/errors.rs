// Copyright © CreditChain Research Team
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This module defines error types used by `CreditChainDB`.
use creditchain_types::state_store::errors::StateViewError;
use std::sync::mpsc::RecvError;
use thiserror::Error;

/// This enum defines errors commonly used among `CreditChainDB` APIs.
#[derive(Clone, Debug, Error)]
pub enum CreditChainDbError {
    /// A requested item is not found.
    #[error("{0} not found.")]
    NotFound(String),
    /// Requested too many items.
    #[error("Too many items requested: at least {0} requested, max is {1}")]
    TooManyRequested(u64, u64),
    #[error("Missing state root node at version {0}, probably pruned.")]
    MissingRootError(u64),
    /// Other non-classified error.
    #[error("CreditChainDB Other Error: {0}")]
    Other(String),
    #[error("CreditChainDB RocksDb Error: {0}")]
    RocksDbIncompleteResult(String),
    #[error("CreditChainDB RocksDB Error: {0}")]
    OtherRocksDbError(String),
    #[error("CreditChainDB bcs Error: {0}")]
    BcsError(String),
    #[error("CreditChainDB IO Error: {0}")]
    IoError(String),
    #[error("CreditChainDB Recv Error: {0}")]
    RecvError(String),
    #[error("CreditChainDB ParseInt Error: {0}")]
    ParseIntError(String),
}

impl From<anyhow::Error> for CreditChainDbError {
    fn from(error: anyhow::Error) -> Self {
        Self::Other(format!("{}", error))
    }
}

impl From<bcs::Error> for CreditChainDbError {
    fn from(error: bcs::Error) -> Self {
        Self::BcsError(format!("{}", error))
    }
}

impl From<RecvError> for CreditChainDbError {
    fn from(error: RecvError) -> Self {
        Self::RecvError(format!("{}", error))
    }
}

impl From<std::io::Error> for CreditChainDbError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(format!("{}", error))
    }
}

impl From<std::num::ParseIntError> for CreditChainDbError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::Other(format!("{}", error))
    }
}

impl From<CreditChainDbError> for StateViewError {
    fn from(error: CreditChainDbError) -> Self {
        match error {
            CreditChainDbError::NotFound(msg) => StateViewError::NotFound(msg),
            CreditChainDbError::Other(msg) => StateViewError::Other(msg),
            _ => StateViewError::Other(format!("{}", error)),
        }
    }
}

impl From<StateViewError> for CreditChainDbError {
    fn from(error: StateViewError) -> Self {
        match error {
            StateViewError::NotFound(msg) => CreditChainDbError::NotFound(msg),
            StateViewError::Other(msg) => CreditChainDbError::Other(msg),
            StateViewError::BcsError(err) => CreditChainDbError::BcsError(err.to_string()),
        }
    }
}
