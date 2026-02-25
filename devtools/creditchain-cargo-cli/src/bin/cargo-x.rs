// Copyright © CreditChain Research Team
// SPDX-License-Identifier: Apache-2.0

use creditchain_cargo_cli::{CreditChainCargoCommand, SelectedPackageArgs};
use clap::Parser;
use std::process::exit;

#[derive(Parser)] // requires `derive` feature
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    #[command(name = "x")]
    CreditChainCargoTool(CreditChainCargoToolArgs),
}

#[derive(Parser)]
struct CreditChainCargoToolArgs {
    #[command(subcommand)]
    cmd: CreditChainCargoCommand,
    #[command(flatten)]
    package_args: SelectedPackageArgs,
}

fn main() {
    let CargoCli::CreditChainCargoTool(args) = CargoCli::parse();
    let CreditChainCargoToolArgs { cmd, package_args } = args;
    let result = cmd.execute(&package_args);

    // At this point, we'll want to print and determine whether to exit for an error code
    match result {
        Ok(_) => {},
        Err(inner) => {
            println!("{}", inner);
            exit(1);
        },
    }
}
