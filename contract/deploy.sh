#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

# https://docs.near.org/tools/near-cli#near-dev-deploy
rm -rf neardev
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/my_near.wasm
