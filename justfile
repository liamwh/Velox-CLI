#!/usr/bin/env -S just --justfile
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set dotenv-load := true
export CARGO_TERM_COLOR := "always"

# Show available commands
default:
    @just --list --justfile {{justfile()}}

# Generate demo gif
tape:
    cargo build
    cp ./target/debug/veloxide ./docs
    cd ./docs && vhs demo.tape 
    rm -rf ./docs/my-demo-app
    rm ./docs/veloxide

