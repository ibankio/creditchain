// Copyright © CreditChain Research Team
// SPDX-License-Identifier: Apache-2.0

use crate::pipeline::signing_phase::CommitSignerProvider;
use creditchain_crypto::bls12381;
use creditchain_types::validator_signer::ValidatorSigner;
use std::sync::Arc;

pub struct DagCommitSigner {
    signer: Arc<ValidatorSigner>,
}

impl DagCommitSigner {
    pub fn new(signer: Arc<ValidatorSigner>) -> Self {
        Self { signer }
    }
}

impl CommitSignerProvider for DagCommitSigner {
    fn sign_commit_vote(
        &self,
        _ledger_info: creditchain_types::ledger_info::LedgerInfoWithSignatures,
        new_ledger_info: creditchain_types::ledger_info::LedgerInfo,
    ) -> Result<bls12381::Signature, creditchain_safety_rules::Error> {
        let signature = self
            .signer
            .sign(&new_ledger_info)
            .map_err(|err| creditchain_safety_rules::Error::SerializationError(err.to_string()))?;

        Ok(signature)
    }
}
