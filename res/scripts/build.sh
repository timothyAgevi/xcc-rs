#!/bin/bash

cargo build --all --target wasm32-unknown-unkown--release
cp target/wasm32-unkown-unknown/release/*.wasm ./res/