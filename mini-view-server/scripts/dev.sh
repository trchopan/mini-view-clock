#!/bin/bash

export DEBUG=1
export RUST_LOG=debug

cargo run -- --note-path $1
