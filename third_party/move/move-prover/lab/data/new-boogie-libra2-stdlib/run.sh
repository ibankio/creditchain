#!/bin/bash

CREDITCHAIN_STD="../../../../../../creditchain-move/framework/creditchain-stdlib/sources"

# Check if the first argument is either "new" or "current"
if [[ "$1" != "new" && "$1" != "current" ]]; then
    echo "Invalid argument. The first argument must be 'new' or 'current'."
    exit 1
fi

# Benchmark per function (with `-f``). `-a` is for including the creditchain-natives.
cargo run --release -p prover-lab -- bench -a -f -c $1_boogie_1.toml $CREDITCHAIN_STD/*.move $CREDITCHAIN_STD/cryptography/*.move $CREDITCHAIN_STD/data_structures/*.move

# Benchmark per module (without `-f`). `-a` is for including the creditchain-natives.
cargo run --release -p prover-lab -- bench -a -c $1_boogie_1.toml $CREDITCHAIN_STD/*.move $CREDITCHAIN_STD/cryptography/*.move $CREDITCHAIN_STD/data_structures/*.move

# Benchmark per function (with `-f``). `-a` is for including the creditchain-natives.
cargo run --release -p prover-lab -- bench -a -f -c $1_boogie_2.toml $CREDITCHAIN_STD/*.move $CREDITCHAIN_STD/cryptography/*.move $CREDITCHAIN_STD/data_structures/*.move

# Benchmark per module (without `-f`). `-a` is for including the creditchain-natives.
cargo run --release -p prover-lab -- bench -a -c $1_boogie_2.toml $CREDITCHAIN_STD/*.move $CREDITCHAIN_STD/cryptography/*.move $CREDITCHAIN_STD/data_structures/*.move

# Benchmark per function (with `-f``). `-a` is for including the creditchain-natives.
cargo run --release -p prover-lab -- bench -a -f -c $1_boogie_3.toml $CREDITCHAIN_STD/*.move $CREDITCHAIN_STD/cryptography/*.move $CREDITCHAIN_STD/data_structures/*.move

# Benchmark per module (without `-f`). `-a` is for including the creditchain-natives.
cargo run --release -p prover-lab -- bench -a -c $1_boogie_3.toml $CREDITCHAIN_STD/*.move $CREDITCHAIN_STD/cryptography/*.move $CREDITCHAIN_STD/data_structures/*.move
