#!/bin/bash -xe

TEMP="$(mktemp)"
API_URL="${CC_DEVNET_API_URL:-${CREDITCHAIN_DEVNET_API_URL:-https://fullnode.devnet.creditchain.org/v1}}"
GENESIS_URL="${CC_DEVNET_GENESIS_URL:-${CREDITCHAIN_DEVNET_GENESIS_URL:-https://devnet.creditchain.org/genesis.blob}}"
WAYPOINT_URL="${CC_DEVNET_WAYPOINT_URL:-${CREDITCHAIN_DEVNET_WAYPOINT_URL:-https://devnet.creditchain.org/waypoint.txt}}"
VALIDATOR_IMAGE="${CC_VALIDATOR_IMAGE:-${CREDITCHAIN_VALIDATOR_IMAGE:-ibankio/creditchain-node}}"
UPGRADE_DOC_URL="${CC_UPGRADE_DOC_URL:-${CREDITCHAIN_UPGRADE_DOC_URL:-https://github.com/ibankio/creditchain}}"

curl "$API_URL" > "$TEMP"

COMMIT="$(jq -r .git_hash "$TEMP")"
CHAIN_ID="$(jq -r .chain_id "$TEMP")"

DIGEST="$(crane digest "$VALIDATOR_IMAGE:devnet_$COMMIT")"
GENESIS_SHA="$(curl "$GENESIS_URL" | shasum -a 256 | awk '{print $1}')"
WAYPOINT="$(curl "$WAYPOINT_URL")"

cat <<EOF

Hey @everyone CreditChain devnet finished release, please update your fullnodes now!

For upgrade, make sure you pulled the latest docker image, or build the rust binary from the latest devnet branch. To confirm:

- Devnet branch commit: $COMMIT
- Docker image tag: devnet_$COMMIT
- Docker image digest: $DIGEST
- genesis.blob sha256: $GENESIS_SHA
- waypoint: $WAYPOINT
- Chain ID: $CHAIN_ID
You can follow the instructions here for upgrade: $UPGRADE_DOC_URL

EOF
