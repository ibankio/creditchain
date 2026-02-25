// Copyright © CreditChain Research Team
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use creditchain_metrics_core::{
    register_int_counter, register_int_counter_vec, register_int_gauge, IntCounter, IntCounterVec,
    IntGauge,
};
use once_cell::sync::Lazy;

pub static CREDITCHAIN_JELLYFISH_LEAF_ENCODED_BYTES: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "creditchain_jellyfish_leaf_encoded_bytes",
        "CreditChain jellyfish leaf encoded bytes in total"
    )
    .unwrap()
});

pub static CREDITCHAIN_JELLYFISH_INTERNAL_ENCODED_BYTES: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "creditchain_jellyfish_internal_encoded_bytes",
        "CreditChain jellyfish total internal nodes encoded in bytes"
    )
    .unwrap()
});

pub static CREDITCHAIN_JELLYFISH_LEAF_COUNT: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "creditchain_jellyfish_leaf_count",
        "Total number of leaves in the latest JMT."
    )
    .unwrap()
});

pub static CREDITCHAIN_JELLYFISH_LEAF_DELETION_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "creditchain_jellyfish_leaf_deletion_count",
        "The number of deletions happened in JMT."
    )
    .unwrap()
});

pub static COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        // metric name
        "creditchain_jellyfish_counter",
        // metric description
        "Various counters for the JellyfishMerkleTree",
        // metric labels (dimensions)
        &["name"],
    )
    .unwrap()
});
