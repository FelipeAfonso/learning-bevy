#!/bin/bash
# build game
cargo build --release --target wasm32-unknown-unknown
# generate deployable wasm
wasm-bindgen --no-typescript --target web --out-dir ./site/public/ --out-name "stupid-spider-game" ./target/wasm32-unknown-unknown/release/learning-bevy.wasm
# copy assets into site
cp ./assets/**/*.{png,ogg,ttf} ./site/public --parents
