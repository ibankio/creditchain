// Copyright © CreditChain Research Team
// SPDX-License-Identifier: Apache-2.0

use super::*;
use creditchain_schemadb::{schema::fuzzing::assert_encode_decode, test_no_panic_decoding};
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_encode_decode(version in any::<u64>(), hash in any::<HashValue>()) {
        assert_encode_decode::<TransactionAccumulatorRootHashSchema>(
            &version,
            &hash,
        );
    }
}

test_no_panic_decoding!(TransactionAccumulatorRootHashSchema);
