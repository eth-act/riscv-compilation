#!/bin/bash

# Create directories
mkdir -p ./assets

# Download asset files
curl -L https://raw.githubusercontent.com/developeruche/stateless-block-exec/main/geth/assets/alloc.json -o ./assets/alloc.json
curl -L https://raw.githubusercontent.com/developeruche/stateless-block-exec/main/geth/assets/env.json -o ./assets/env.json
curl -L https://raw.githubusercontent.com/developeruche/stateless-block-exec/main/geth/assets/exp.json -o ./assets/exp.json
curl -L https://raw.githubusercontent.com/developeruche/stateless-block-exec/main/geth/assets/tx.json -o ./assets/tx.json

echo "Download complete. Files saved to assets directory."
