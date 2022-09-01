#!/usr/bin/env bash

cargo build --target wasm32-unknown-unknown --release

# NOTE if no '--force', it will deploy the contract to the previous address,
# and the `$CONTRACT_NAME new` will fail by `The contract has already been initialized`
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/hello.wasm --force

source ./neardev/dev-account.env

near call $CONTRACT_NAME new --accountId $CONTRACT_NAME

near call $CONTRACT_NAME set_name '{"name":"rust2"}' --accountId $CONTRACT_NAME

near view $CONTRACT_NAME hello --accountId $CONTRACT_NAME