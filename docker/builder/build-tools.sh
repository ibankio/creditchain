#!/bin/bash
# Copyright (c) CreditChain
# SPDX-License-Identifier: Apache-2.0
set -e

PROFILE=cli

echo "Building tools and services docker images"
echo "PROFILE: $PROFILE"
echo "CARGO_TARGET_DIR: $CARGO_TARGET_DIR"

# Build all the rust binaries
cargo build --locked --profile=$PROFILE \
    -p creditchain \
    -p creditchain-backup-cli \
    -p creditchain-faucet-service \
    -p creditchain-fn-check-client \
    -p creditchain-node-checker \
    -p creditchain-openapi-spec-generator \
    -p creditchain-telemetry-service \
    -p creditchain-keyless-pepper-service \
    -p creditchain-debugger \
    -p creditchain-transaction-emitter \
    -p creditchain-api-tester \
    "$@"

# After building, copy the binaries we need to `dist` since the `target` directory is used as docker cache mount and only available during the RUN step
BINS=(
    creditchain
    creditchain-faucet-service
    creditchain-node-checker
    creditchain-openapi-spec-generator
    creditchain-telemetry-service
    creditchain-keyless-pepper-service
    creditchain-fn-check-client
    creditchain-debugger
    creditchain-transaction-emitter
    creditchain-api-tester
)

mkdir dist

for BIN in "${BINS[@]}"; do
    cp $CARGO_TARGET_DIR/$PROFILE/$BIN dist/$BIN
done

# Build the CreditChain Move framework and place it in dist. It can be found afterwards in the current directory.
echo "Building the CreditChain Move framework..."
(cd dist && cargo run --locked --profile=$PROFILE --package creditchain-framework -- release)
