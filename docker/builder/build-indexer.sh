#!/bin/bash
# Copyright (c) CreditChain
# SPDX-License-Identifier: Apache-2.0
set -e

PROFILE=${PROFILE:-release}

echo "Building indexer and related binaries"
echo "PROFILE: $PROFILE"

echo "CARGO_TARGET_DIR: $CARGO_TARGET_DIR"

# Build all the rust binaries
cargo build --locked --profile=$PROFILE \
    -p creditchain-indexer-grpc-cache-worker \
    -p creditchain-indexer-grpc-file-store \
    -p creditchain-indexer-grpc-data-service \
    -p creditchain-nft-metadata-crawler \
    -p creditchain-indexer-grpc-file-checker \
    -p creditchain-indexer-grpc-data-service-v2 \
    -p creditchain-indexer-grpc-manager \
    "$@"

# After building, copy the binaries we need to `dist` since the `target` directory is used as docker cache mount and only available during the RUN step
BINS=(
    creditchain-indexer-grpc-cache-worker
    creditchain-indexer-grpc-file-store
    creditchain-indexer-grpc-data-service
    creditchain-nft-metadata-crawler
    creditchain-indexer-grpc-file-checker
    creditchain-indexer-grpc-data-service-v2
    creditchain-indexer-grpc-manager
)

mkdir dist

for BIN in "${BINS[@]}"; do
    cp $CARGO_TARGET_DIR/$PROFILE/$BIN dist/$BIN
done
