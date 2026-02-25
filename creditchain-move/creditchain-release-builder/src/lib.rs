// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

pub mod components;
pub mod simulate;
mod utils;
pub mod validate;

pub use components::{ExecutionMode, ReleaseConfig, ReleaseEntry};
use once_cell::sync::{Lazy, OnceCell};
use std::{
    env,
    path::{Path, PathBuf},
};

// Update me after branch cut.
const RELEASE_CONFIG: &str = include_str!("../data/release.yaml");

static CURRENT_RELEASE_CONFIG: Lazy<ReleaseConfig> =
    Lazy::new(|| ReleaseConfig::parse(RELEASE_CONFIG).expect("YAML NOT PARSABLE"));

/// Returns the release bundle with which the last testnet was build or updated.
pub fn current_release_config() -> &'static ReleaseConfig {
    &CURRENT_RELEASE_CONFIG
}

static CREDITCHAIN_CORE_PATH: OnceCell<PathBuf> = OnceCell::new();

fn creditchain_core_path_at_compile_time() -> PathBuf {
    let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    path.pop();
    path.pop();
    path = path.canonicalize().unwrap();
    path
}

pub fn initialize_creditchain_core_path(overriden_path: Option<PathBuf>) {
    if let Some(path) = overriden_path {
        CREDITCHAIN_CORE_PATH.set(path).unwrap();
    } else {
        CREDITCHAIN_CORE_PATH
            .set(creditchain_core_path_at_compile_time())
            .unwrap();
    };
}

pub(crate) fn creditchain_core_path() -> PathBuf {
    CREDITCHAIN_CORE_PATH
        .get_or_init(creditchain_core_path_at_compile_time)
        .clone()
}

pub(crate) fn creditchain_framework_path() -> PathBuf {
    let mut path = creditchain_core_path();
    path.push("../../framework/creditchain-framework");
    path
}
