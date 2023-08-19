#!/usr/bin/env bash

set -euxo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
ROOT_DIR="$SCRIPT_DIR/.."

git submodule update --init --recursive 

FINSCHIA_SDK_REV=${1:-main}
WASMD_REV=${2:-main}

#####################################
## Update and rebuild FINSCHIA-std ##
#####################################

# update revision in proto-build main.rs
PROTO_BUILD_MAIN_RS="$SCRIPT_DIR/../packages/proto-build/src/main.rs"

# use @ as a separator to avoid confusion on input like "origin/main"
sed -i -- "s@const FINSCHIA_SDK_REV: \&str = \".*\";@const FINSCHIA_SDK_REV: \&str = \"$FINSCHIA_SDK_REV\";@g" "$PROTO_BUILD_MAIN_RS"
sed -i -- "s@const WASMD_REV: \&str = \".*\";@const WASMD_REV: \&str = \"$WASMD_REV\";@g" "$PROTO_BUILD_MAIN_RS"

git diff

# rebuild FINSCHIA-std
cd "$SCRIPT_DIR/../packages/proto-build/" && cargo run -- --update-deps

# replace type Result to Result1 for cosmos/base/abci/v1beta1.rs
PROTO_COSMOS_ABCI_RS="$SCRIPT_DIR/../packages/finschia-std/src/types/cosmos/base/abci/v1beta1.rs"
sed -i -- "s@pub struct Result {@pub struct Result1 {@g" "$PROTO_COSMOS_ABCI_RS"
sed -i -- "s@Option<Result>@Option<Result1>@g" "$PROTO_COSMOS_ABCI_RS"

# replace type Result to Result1 for cosmos/tx/v1beta1.rs
PROTO_COSMOS_TX_RS="$SCRIPT_DIR/../packages/finschia-std/src/types/cosmos/tx/v1beta1.rs"
sed -i -- "s@Option<super::super::base::abci::v1beta1::Result>@Option<super::super::base::abci::v1beta1::Result1>@g" "$PROTO_COSMOS_TX_RS"
