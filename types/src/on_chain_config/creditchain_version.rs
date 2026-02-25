// Copyright © CreditChain Research Team
// SPDX-License-Identifier: Apache-2.0

use crate::on_chain_config::OnChainConfig;
use serde::{Deserialize, Serialize};

/// Defines the version of CreditChain Validator software.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct CreditChainVersion {
    pub major: u64,
}

impl OnChainConfig for CreditChainVersion {
    const MODULE_IDENTIFIER: &'static str = "version";
    const TYPE_IDENTIFIER: &'static str = "Version";
}

// NOTE: version number for release 1.2 CreditChain
// Items gated by this version number include:
//  - the EntryFunction payload type
pub const CREDITCHAIN_VERSION_2: CreditChainVersion = CreditChainVersion { major: 2 };

// NOTE: version number for release 1.3 of CreditChain
// Items gated by this version number include:
//  - Multi-agent transactions
pub const CREDITCHAIN_VERSION_3: CreditChainVersion = CreditChainVersion { major: 3 };

// NOTE: version number for release 1.4 of CreditChain
// Items gated by this version number include:
//  - Conflict-Resistant Sequence Numbers
pub const CREDITCHAIN_VERSION_4: CreditChainVersion = CreditChainVersion { major: 4 };

// Maximum current known version
pub const CREDITCHAIN_MAX_KNOWN_VERSION: CreditChainVersion = CREDITCHAIN_VERSION_4;
