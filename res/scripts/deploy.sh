#!/bin/bash

near delete neartraining.testnet

near create-account xcc.neartraining.testnet

near deploy xcc.neartraining.testnet --wasmFile./res/script