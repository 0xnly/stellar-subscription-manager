#!/bin/bash

# StellarSubs Deploy Script

echo "🚀 Building contract..."
cargo build --target wasm32-unknown-unknown --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"

echo "📦 Optimizing WASM..."
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/stellar_subs.wasm

echo "🌐 Deploying to Testnet..."
CONTRACT_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stellar_subs.wasm \
  --source alice \
  --network testnet 2>&1 | tail -n 1)

if [ $? -eq 0 ]; then
    echo "✅ Contract deployed successfully!"
    echo "📝 Contract ID: $CONTRACT_ID"
    echo ""
    echo "Save this contract ID for future interactions!"
else
    echo "❌ Deployment failed!"
    exit 1
fi
