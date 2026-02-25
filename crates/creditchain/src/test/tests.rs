// Copyright © CreditChain Research Team
// SPDX-License-Identifier: Apache-2.0

use crate::{
    move_tool::{ArgWithType, FunctionArgType},
    CliResult, Tool,
};
use clap::Parser;
use std::str::FromStr;

/// In order to ensure that there aren't duplicate input arguments for untested CLI commands,
/// we call help on every command to ensure it at least runs
#[tokio::test]
async fn ensure_every_command_args_work() {
    assert_cmd_not_panic(&["creditchain"]).await;

    assert_cmd_not_panic(&["creditchain", "account"]).await;
    assert_cmd_not_panic(&["creditchain", "account", "create", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "account", "create-resource-account", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "account", "fund-with-faucet", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "account", "list", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "account", "lookup-address", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "account", "rotate-key", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "account", "transfer", "--help"]).await;

    assert_cmd_not_panic(&["creditchain", "config"]).await;
    assert_cmd_not_panic(&["creditchain", "config", "generate-shell-completions", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "config", "init", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "config", "set-global-config", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "config", "show-global-config"]).await;
    assert_cmd_not_panic(&["creditchain", "config", "show-profiles"]).await;

    assert_cmd_not_panic(&["creditchain", "genesis"]).await;
    assert_cmd_not_panic(&["creditchain", "genesis", "generate-genesis", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "genesis", "generate-keys", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "genesis", "generate-layout-template", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "genesis", "set-validator-configuration", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "genesis", "setup-git", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "genesis", "generate-admin-write-set", "--help"]).await;

    assert_cmd_not_panic(&["creditchain", "governance"]).await;
    assert_cmd_not_panic(&["creditchain", "governance", "execute-proposal", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "governance", "generate-upgrade-proposal", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "governance", "propose", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "governance", "vote", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "governance", "delegation_pool", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "governance", "delegation_pool", "vote", "--help"]).await;
    assert_cmd_not_panic(&[
        "creditchain",
        "governance",
        "delegation_pool",
        "propose",
        "--help",
    ])
    .await;

    assert_cmd_not_panic(&["creditchain", "info"]).await;

    assert_cmd_not_panic(&["creditchain", "init", "--help"]).await;

    assert_cmd_not_panic(&["creditchain", "key"]).await;
    assert_cmd_not_panic(&["creditchain", "key", "generate", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "key", "extract-peer", "--help"]).await;

    assert_cmd_not_panic(&["creditchain", "move"]).await;
    assert_cmd_not_panic(&["creditchain", "move", "clean", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "move", "compile", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "move", "compile-script", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "move", "decompile", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "move", "disassemble", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "move", "download", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "move", "init", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "move", "list", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "move", "prove", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "move", "publish", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "move", "run", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "move", "run-script", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "move", "test", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "move", "transactional-test", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "move", "view", "--help"]).await;

    assert_cmd_not_panic(&["creditchain", "node"]).await;
    assert_cmd_not_panic(&["creditchain", "node", "check-network-connectivity", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "node", "get-stake-pool", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "node", "analyze-validator-performance", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "node", "bootstrap-db-from-backup", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "node", "initialize-validator", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "node", "join-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "node", "leave-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "node", "run-local-testnet", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "node", "show-validator-config", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "node", "show-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "node", "show-validator-stake", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "node", "update-consensus-key", "--help"]).await;
    assert_cmd_not_panic(&[
        "creditchain",
        "node",
        "update-validator-network-addresses",
        "--help",
    ])
    .await;

    assert_cmd_not_panic(&["creditchain", "stake"]).await;
    assert_cmd_not_panic(&["creditchain", "stake", "add-stake", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "stake", "increase-lockup", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "stake", "initialize-stake-owner", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "stake", "set-delegated-voter", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "stake", "set-operator", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "stake", "unlock-stake", "--help"]).await;
    assert_cmd_not_panic(&["creditchain", "stake", "withdraw-stake", "--help"]).await;
}

/// Ensure we can parse URLs for args
#[tokio::test]
async fn ensure_can_parse_args_with_urls() {
    let result = ArgWithType::from_str("string:https://creditchain.io").unwrap();
    matches!(result._ty, FunctionArgType::String);
    assert_eq!(
        result.arg,
        bcs::to_bytes(&"https://creditchain.io".to_string()).unwrap()
    );
}

async fn assert_cmd_not_panic(args: &[&str]) {
    // When a command fails, it will have a panic in it due to an improperly setup command
    // thread 'main' panicked at 'Command propose: Argument names must be unique, but 'assume-yes' is
    // in use by more than one argument or group', ...

    match run_cmd(args).await {
        Ok(inner) => assert!(
            !inner.contains("panic"),
            "Failed to not panic cmd {}: {}",
            args.join(" "),
            inner
        ),
        Err(inner) => assert!(
            !inner.contains("panic"),
            "Failed to not panic cmd {}: {}",
            args.join(" "),
            inner
        ),
    }
}

async fn run_cmd(args: &[&str]) -> CliResult {
    let tool: Tool = Tool::try_parse_from(args).map_err(|msg| msg.to_string())?;
    tool.execute().await
}
