// Copyright © CreditChain Research Team
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use creditchain_forge::{forge_main, ForgeConfig, InitialVersion, LocalFactory, Options, Result};
use creditchain_testcases::compatibility_test::SimpleValidatorUpgrade;
use std::num::NonZeroUsize;

fn main() -> Result<()> {
    ::creditchain_logger::Logger::init_for_testing();

    let tests = ForgeConfig::default()
        .with_initial_validator_count(NonZeroUsize::new(4).unwrap())
        .with_initial_version(InitialVersion::Oldest)
        .add_network_test(SimpleValidatorUpgrade);

    let options = Options::parse();
    forge_main(
        tests,
        LocalFactory::with_upstream_merge_base_and_workspace()?,
        &options,
    )
}
